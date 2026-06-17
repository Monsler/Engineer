use serde_json::{Value, json};
use reqwest;

/* 
--------------------------------------------------
LLMCore is a service trait everyone inherits from.
Coded by Monsler at 16/06/2026
--------------------------------------------------
*/

pub trait LLMCore<'llm, 'a> {
    fn prompt(&mut self, request: &str);
    fn clear_context(&mut self);
    fn set_system_prompt(&mut self, prompt: &str);
    fn history(&self) -> &Value;
    fn set_api_key(&mut self, key: &'a str);
}

pub struct G4Free<'a> {
    context: Value,
    api_key: &'a str
}

impl<'a> G4Free<'a> {
    pub fn new(api_key: &'a str) -> Self {
        Self { context: json!([]), api_key }
    }
    
    fn receive_array(&mut self) -> &mut Vec<Value> {
        if !self.context.is_array() {
            self.context = json!([]);
        }

        self.context.as_array_mut().unwrap()
    }

}

impl<'llm, 'a> LLMCore<'llm, 'a> for G4Free<'a> {
    fn prompt(&mut self, request: &str) {
       let array = self.receive_array();

       array.push(json!({
                "role": "user",
                "content": request
        }))
    }

    fn set_api_key(&mut self, key: &'a str) {
        self.api_key = key;
    }

    fn clear_context(&mut self) {
        self.context = json!([]);
    }

    fn set_system_prompt(&mut self, prompt: &str) {
        let array = self.receive_array();
        
        array.insert(0, json!({
            "role": "system",
            "content": prompt
        }));
    }

    fn history(&self) -> &Value {
        return &self.context
    }
}