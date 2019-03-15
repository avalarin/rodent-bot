mod telegram;

pub use telegram::*;

#[derive(Debug, PartialEq)]
pub enum SideEffect {
    Telegram(telegram::TelegramSideEffect),
    Dummy(String)
}

pub struct SideEffectsSet {
    pub items: Vec<SideEffect>,
}

pub enum SideEffectReduceAction {
    Skip,
    Break,
    Pop
}

impl SideEffectsSet {
    pub fn empty() -> Self {
        SideEffectsSet {
            items: vec![]
        }
    }

    pub fn put<T: Into<SideEffect>>(mut self, side_effect: T) -> Self {
        self.items.push(side_effect.into());
        self
    }

    pub fn reduce<F: Fn(&SideEffect, usize) -> SideEffectReduceAction>(mut self, func: F) -> Self {
        for i in (0..self.items.len()).rev() {
            let item = &self.items[i];
            match func(item, i) {
                SideEffectReduceAction::Skip => {},
                SideEffectReduceAction::Break => {
                    break;
                },
                SideEffectReduceAction::Pop => {
                    self.items.remove(i);
                }
            }
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::SideEffectReduceAction::Break;
    use super::SideEffectReduceAction::Skip;
    use super::SideEffectReduceAction::Pop;

    #[test]
    fn test_reduce() {
        let set = SideEffectsSet::empty()
            .put(SideEffect::Dummy("test1".to_string()))
            .put(SideEffect::Dummy("test2".to_string()))
            .put(SideEffect::Dummy("test3".to_string()))
            .put(SideEffect::Dummy("test4".to_string()))
            .reduce(|se, _| match se {
                SideEffect::Dummy(str) if str == "test1" => Pop,
                SideEffect::Dummy(str) if str == "test2" => Break,
                SideEffect::Dummy(str) if str == "test3" => Pop,
                _ => Skip,
            });

        assert_eq!(set.items, vec![
            SideEffect::Dummy("test1".to_string()),
            SideEffect::Dummy("test2".to_string()),
            SideEffect::Dummy("test4".to_string())
        ]);
    }

}