use crate::net;

/*
------------------------------
Engineer needed infrastructure
Coded by Monsler at 17/06/2026
------------------------------
*/

pub struct Engineer<'llm, 'a> {
    core: Box<dyn net::LLMCore<'llm, 'a> + Send + Sync>,
}

impl<'llm, 'a, T> From<Box<T>> for Engineer<'llm, 'a> where T: net::LLMCore<'llm, 'a> + 'static + Send + Sync {
    fn from(value: Box<T>) -> Self {
        Self {core: value}
    }
}

impl<'llm, 'a> Engineer<'llm, 'a> {
    pub fn set_api_key(&mut self, key: &'a str) {
        self.core.as_mut().set_api_key(key);
    }
}