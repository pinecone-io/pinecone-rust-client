use crate::openapi::apis::inference_api;
use crate::openapi::models::{EmbedRequest, EmbedRequestInputsInner};
use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;

pub use crate::openapi::models::{EmbedRequestParameters, EmbeddingsList};

impl PineconeClient {
    /// Generate embeddings for input data.
    ///
    /// ### Arguments
    /// * `model: &str` - The model to use for embedding.
    /// * `parameters: Option<EmbedRequestParameters>` - Model-specific parameters.
    /// * `inputs: &Vec<&str>` - The input data to embed.
    ///
    /// ### Return
    /// * `Result<EmbeddingsList, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), pinecone_sdk::utils::errors::PineconeError> {
    ///
    /// let pinecone = PineconeClient::new(None, None, None, None)?;
    /// let response = pinecone.embed("multilingual-e5-large", None, &vec!["Hello, world!"]).await.expect("Failed to embed");
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn embed(
        &self,
        model: &str,
        parameters: Option<EmbedRequestParameters>,
        inputs: &Vec<&str>,
    ) -> Result<EmbeddingsList, PineconeError> {
        let request = EmbedRequest {
            model: model.to_string(),
            parameters: parameters.map(|x| Box::new(x)),
            inputs: inputs
                .iter()
                .map(|&x| EmbedRequestInputsInner {
                    text: Some(x.to_string()),
                })
                .collect(),
        };

        let res = inference_api::embed(&self.openapi_config, Some(request))
            .await
            .map_err(|e| PineconeError::from(e))?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use tokio;

    #[tokio::test]
    async fn test_embed() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/embed");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "model": "multilingual-e5-large",
                        "data": [
                          {"values": [0.01849365234375, -0.003767013549804688, -0.037261962890625, 0.0222930908203125]}
                        ],
                        "usage": {"total_tokens": 1632}
                    }
                    "#,
                );
        });

        let client = PineconeClient::new(None, Some(server.base_url().as_str()), None, None)?;
        let response = client
            .embed("multilingual-e5-large", None, &vec!["Hello, world!"])
            .await
            .expect("Failed to embed");

        mock.assert();

        assert_eq!(response.model.unwrap(), "multilingual-e5-large");
        assert_eq!(response.data.unwrap().len(), 1);
        assert_eq!(response.usage.unwrap().total_tokens, Some(1632));

        Ok(())
    }

    async fn test_embed_invalid_arguments() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/embed");
            then.status(400)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": {
                          "code": "INVALID_ARGUMENT",
                          "message": "Invalid parameter value input_type='bad-parameter' for model 'multilingual-e5-large', must be one of [query, passage]"
                        },
                        "status": 400
                      }
                    "#,
                );
        });

        let client = PineconeClient::new(None, None, None, None).unwrap();

        let parameters = EmbedRequestParameters {
            input_type: Some("bad-parameter".to_string()),
            truncate: Some("bad-parameter".to_string()),
        };

        let _ = client
            .embed(
                "multilingual-e5-large",
                Some(parameters),
                &vec!["Hello, world!"],
            )
            .await
            .expect_err("Expected to fail embedding with invalid arguments");

        mock.assert();

        Ok(())
    }
}
