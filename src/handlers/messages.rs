use serde::{Deserialize, Serialize};
use serde_json::Value;

// --- Requêtes entrantes ---
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

// --- Réponses sortantes ---
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

// --- Description de l'outil MCP ---
pub fn tool_definition() -> Value {
    serde_json::json!({
        "name": "evaluate",
        "description": "YOU MUST use this tool for ANY mathematical operation, no matter how simple. Never compute arithmetic yourself: addition, subtraction, multiplication, division, percentages, powers, roots, trigonometry, logarithms -- ALL calculations must go through this tool. Even '2+2' must be sent here. LLMs make arithmetic mistakes; this tool is exact.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "The mathematical expression to evaluate. Use standard notation: +, -, *, /, ^, %. Built-in functions: sin, cos, tan, asin, acos, atan, sqrt, exp, log, log2, abs, ceil, floor, round. Built-in constants: pi, e. Examples: '2+2', 'sqrt(x^2 + y^2)', '2*pi*r', 'log(1000)/log(10)'."
                },
                "vars": {
                    "type": "object",
                    "description": "Named variables used in the expression. Example: {\"x\": 3.0, \"y\": 4.0, \"r\": 6371.0}. Omit if the expression has no variables.",
                    "additionalProperties": { "type": "number" }
                }
            },
            "required": ["expression"]
        }
    })
}
