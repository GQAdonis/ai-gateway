# Dify Provider

The Dify provider enables integration with Dify's AI infrastructure through an OpenAI-compatible API interface. This provider supports chat flows and workflows with different capabilities for each mode.

## Configuration

### Required Headers

| Header | Required | Description |
|--------|----------|-------------|
| `x-provider` | Yes | Must be set to `dify` |
| `x-dify-base-url` | Yes | The base URL of your Dify instance (e.g., `https://your-dify-instance.com`) |
| `Authorization` | Yes | Bearer token with your Dify API key |

### Optional Headers

| Header | Required | Description |
|--------|----------|-------------|
| `x-dify-workflow-id` | No | When present, routes the request to Dify's workflow endpoint (`/v1/workflows/{id}/run`). Workflows support document files but not images. |
| `x-chat-id` | No | Used for chat flows to maintain conversation context. Only valid when x-dify-workflow-id is not present. |

## Endpoint Selection

The provider automatically routes requests to different Dify endpoints based on the headers:

1. With `x-dify-workflow-id`:
   - Routes to `/v1/workflows/{id}/run`
   - Supports document files but not images
   - Used for executing specific workflows
   - Ignores `x-chat-id` header

2. Without `x-dify-workflow-id`:
   - Routes to `/v1/chat-messages`
   - Supports images with validation
   - Used for chat flows
   - Can include `x-chat-id` for conversation continuity

## Chat Flow Examples

### Basic Chat
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR-DIFY-API-KEY" \
  -H "x-provider: dify" \
  -H "x-dify-base-url: https://your-dify-instance.com" \
  -H "x-chat-id: optional-conversation-id" \
  -d '{
    "messages": [{"role": "user", "content": "Hello!"}],
    "stream": true
  }'
```

### Chat with Image
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR-DIFY-API-KEY" \
  -H "x-provider: dify" \
  -H "x-dify-base-url: https://your-dify-instance.com" \
  -H "x-chat-id: optional-conversation-id" \
  -d '{
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "Analyze this image"
          },
          {
            "type": "image_url",
            "image_url": {
              "url": "https://example.com/image.jpg"
            }
          }
        ]
    }],
    "stream": true
  }'
```

## Workflow Examples

### Basic Workflow
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR-DIFY-API-KEY" \
  -H "x-provider: dify" \
  -H "x-dify-base-url: https://your-dify-instance.com" \
  -H "x-dify-workflow-id: your-workflow-id" \
  -d '{
    "messages": [{"role": "user", "content": "Process this request"}],
    "stream": true
  }'
```

### Workflow with Document
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR-DIFY-API-KEY" \
  -H "x-provider: dify" \
  -H "x-dify-base-url: https://your-dify-instance.com" \
  -H "x-dify-workflow-id: your-workflow-id" \
  -d '{
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "Analyze this document"
          },
          {
            "type": "file",
            "file": {
              "type": "document",
              "transfer_method": "remote_url",
              "url": "https://example.com/document.pdf"
            }
          }
        ]
    }],
    "stream": true
  }'
```

## File Support

### Chat Flow File Support
- Images (JPEG, PNG, GIF, WebP)
- Maximum size: 10MB
- Transfer methods:
  - Remote URL
  - Base64 encoding
  - File reference (requires prior upload)

### Workflow File Support
- Documents (PDF, DOCX, TXT)
- Maximum size: 50MB
- Transfer methods:
  - Remote URL
  - Base64 encoding
  - File reference (requires prior upload)

### File Upload
Before using file references, upload the file to Dify:
```bash
curl -X POST 'https://your-dify-instance.com/v1/files/upload' \
  -H 'Authorization: Bearer YOUR-API-KEY' \
  -F 'file=@path/to/file.jpg'
```

Response:
```json
{
    "id": "file_id_12345",
    "name": "file.jpg",
    "url": "internal_file_reference"
}
```

Then use the file ID in your request:
```json
{
    "type": "file",
    "file": {
        "type": "image",
        "transfer_method": "local_file",
        "url": "file_id_12345"
    }
}
```

## Tool Calling

Both chat flows and workflows support tool calling:

```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR-DIFY-API-KEY" \
  -H "x-provider: dify" \
  -H "x-dify-base-url: https://your-dify-instance.com" \
  -d '{
    "messages": [{"role": "user", "content": "What's the weather?"}],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "get_weather",
          "description": "Get current weather",
          "parameters": {
            "type": "object",
            "properties": {
              "location": {
                "type": "string",
                "description": "City name"
              }
            },
            "required": ["location"]
          }
        }
      }
    ]
  }'
```

## Response Formats

### Streaming Response
```json
{
    "event": "message_stream",
    "id": "thought-id",
    "message_id": "message-id",
    "conversation_id": "session-id",
    "answer": "Partial response content",
    "created_at": 1705395332
}
```

### Blocking Response
```json
{
    "id": "thought-id",
    "message_id": "message-id",
    "conversation_id": "session-id",
    "mode": "chat",
    "answer": "Complete response content",
    "metadata": {
        // Additional metadata
    },
    "created_at": 1705395332
}
```

## Error Handling

```json
{
    "error": {
        "message": "Error description",
        "type": "error_type",
        "code": "error_code"
    }
}
```

Common error codes:
- 400: Invalid request format
- 401: Invalid API key
- 404: Workflow not found
- 413: File size too large
- 415: Unsupported file type
- 429: Rate limit exceeded

## OpenAI SDK Example

```typescript
import OpenAI from 'openai';

// Chat Flow
const difyChat = new OpenAI({
    apiKey: 'your-dify-api-key',
    baseURL: 'http://localhost:3000/v1',
    defaultHeaders: {
        'x-provider': 'dify',
        'x-dify-base-url': 'https://your-dify-instance.com',
        'x-chat-id': 'optional-chat-id'
    }
});

// Workflow
const difyWorkflow = new OpenAI({
    apiKey: 'your-dify-api-key',
    baseURL: 'http://localhost:3000/v1',
    defaultHeaders: {
        'x-provider': 'dify',
        'x-dify-base-url': 'https://your-dify-instance.com',
        'x-dify-workflow-id': 'your-workflow-id'
    }
});

// Example usage
const response = await difyChat.chat.completions.create({
    messages: [{ role: 'user', content: 'Hello!' }],
    stream: true
});
