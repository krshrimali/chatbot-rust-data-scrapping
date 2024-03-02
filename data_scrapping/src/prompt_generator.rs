use openai_api_rust::chat::*;
use openai_api_rust::OpenAI;

#[derive(Default)]
pub struct PromptGenerator {
    openai_api: Option<OpenAI>,
    url: String,
}

#[derive(Default)]
pub struct Dataset {
    data: Vec<(String, String)>,
}

impl PromptGenerator {
    pub fn init(&mut self) -> PromptGenerator {
        let auth = Auth::from_env().unwrap();
        Self {
            openai_api: Some(OpenAI::new(auth.clone(), "https://api.openai.com/v1/")),
            url: "https://api.openai.com/v1/".to_string(),
        }
    }

    pub fn generate_output(&mut self, text: &str) -> String {
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

        let result = self.openai_api.as_ref().unwrap().chat_completion_create(&body);

        let choice = result.unwrap().choices;
        // FIXME: Untested code
        let message = &choice[0].message.as_ref().unwrap();

        message.content.clone()
    }

    pub fn process_output_text_into_dataset(&mut self, output_from_model: String) -> Dataset {
        Dataset::default()
    }
}

impl Dataset {
    pub fn extend(&mut self, another_dataset: Dataset) {
        self.data.extend(another_dataset.data);
    }
}

// mod test {
//     #[test]
//     fn test_prompt_generator_generate_output() {
//         // mock PromptGenerator's auth and openai_api
//     }
// }
