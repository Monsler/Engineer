use serde_json::{Value, json};
use reqwest;

/* 
--------------------------------------------------
LLMCore is a service trait everyone inherits from.
Coded by Monsler at 16/06/2026
--------------------------------------------------
*/

trait LLMCore<'llm> {
    fn prompt(&mut self, request: &str);
    fn clear_context(&mut self);
    fn set_system_prompt(prompt: &str);
    fn history(&self) -> &Value;
}

struct G4Free {
    context: Value
}

impl G4Free {
    pub fn new() -> Self {
        Self { context: json!([]) }
    }
}

impl<'llm, 'a> LLMCore<'llm> for G4Free {
    fn prompt(&mut self, request: &str) {
        if let Some(array) = self.context.as_array_mut() {
            array.push(json!({
                "role": "user",
                "content": request
            }))
        }
    }

    fn clear_context(&mut self) {
        self.context = json!([]);
    }

    fn set_system_prompt(prompt: &str) {
        todo!()
    }

    fn history(&self) -> &Value {
        return &self.context
    }
}