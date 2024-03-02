use openai_api_rust::chat::*;
use openai_api_rust::*;

struct PromptGenerator {
    auth: Auth,
    openai_api: OpenAI,
    url: String,
}

impl PromptGenerator {
    fn init(&mut self) {
        self.auth = Auth::from_env().unwrap();
        self.openai_api = OpenAI::new(self.auth.clone(), "https://api.openai.com/v1/");
    }

    fn generate_output(&mut self, text: &str) -> String {
        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: Some(7),
            temperature: Some(0_f32),
            top_p: Some(0_f32),
            n: Some(2),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: vec![Message {
                role: Role::User,
                content: text.to_string(),
            }],
        };

        let result = self.openai_api.chat_completion_create(&body);

        let choice = result.unwrap().choices;
        // FIXME: Untested code
        let message = &choice[0].message.as_ref().unwrap();

        message.content.clone()
    }
}

// mod test {
//     #[test]
//     fn test_prompt_generator_generate_output() {
//         // mock PromptGenerator's auth and openai_api
//     }
// }
