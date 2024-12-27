# Overview

## API Specification

The completion messages API endpoint accepts POST requests at:
```bash
POST https://dify-api.prometheus-platform.io/v1/completion-messages
```

## Authentication
Include API key in request header:
```http
Authorization: Bearer YOUR-API-KEY
Content-Type: application/json
```

## Request Formats

### Blocking Mode
```json
{
    "inputs": {
        "text": "Your input text"
    },
    "response_mode": "blocking",
    "user": "user-identifier"
}
```

### Streaming Mode
```json
{
    "inputs": {
        "text": "Your input text"
    },
    "response_mode": "streaming",
    "user": "user-identifier"
}
```

## Response Formats

### Blocking Mode Response
```json
{
    "id": "thought-id",
    "message_id": "message-id",
    "conversation_id": "session-id",
    "mode": "completion",
    "answer": "Complete response content",
    "metadata": {
        // Additional metadata
    },
    "created_at": 1705395332
}
```

### Streaming Mode Events

**1. Message Start Event:**
```json
{
    "event": "message_start",
    "id": "thought-id",
    "message_id": "message-id",
    "conversation_id": "session-id",
    "created_at": 1705395332
}
```

**2. Message Stream Event:**
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

**3. Message End Event:**
```json
{
    "event": "message_end",
    "id": "thought-id",
    "message_id": "message-id",
    "conversation_id": "session-id",
    "answer": "Complete response content",
    "metadata": {
        // Additional metadata
    },
    "created_at": 1705395332
}
```

## Error Response
```json
{
    "error": {
        "message": "Error description",
        "type": "error_type",
        "code": "error_code"
    }
}
```

## Processing Considerations

- Streaming responses must be processed chunk by chunk as they arrive
- Blocking calls will wait for complete response before returning
- User identifier should be consistent across related requests
- Implement proper error handling for both modes
- Consider implementing request timeouts
- Store conversation_id if maintaining conversation context

Citations:
[1] https://pkg.go.dev/github.com/kervinchang/dify-go
[2] https://www.restack.io/p/dify-answer-open-api-examples-cat-ai
[3] https://docs.dify.ai/guides/application-publishing/developing-with-apis
[4] https://docs.dify.ai/guides/extension/api-based-extension
[5] https://www.restack.io/p/dify-add-option-for-blocking-in-api-call-to-chat-agent
[6] https://www.restack.io/p/dify-answer-open-api-file-example-cat-ai
[7] https://pkg.go.dev/github.com/bswaterb/dify-go-sdk
[8] https://github.com/langgenius/dify/issues/6047
[9] https://github.com/langgenius/dify/blob/main/api/controllers/web/completion.py
[10] https://swanhub.co/hm666/dify/blob/main/api/core/model_runtime/model_providers/openai_api_compatible/llm/llm.py

## Response Processing

## Blocking Response Format

When using `response_mode: "blocking"`, you'll receive a single JSON response:

```json
{
    "id": "cm_12345",
    "message_id": "msg_67890",
    "answer": "The complete generated response text",
    "metadata": {
        "usage": {
            "prompt_tokens": 124,
            "completion_tokens": 356,
            "total_tokens": 480
        },
        "finish_reason": "stop",
        "model": "gpt-4",
        "generated_at": "2024-12-26T18:00:00Z"
    },
    "created_at": 1703624400
}
```

## Streaming Events Sequence

### 1. Message Start Event
```json
{
    "event": "message_start",
    "id": "cm_12345",
    "message_id": "msg_67890",
    "created_at": 1703624400,
    "metadata": {
        "model": "gpt-4",
        "session_start": "2024-12-26T18:00:00Z"
    }
}
```

### 2. Message Stream Events
Multiple stream events containing partial content:
```json
{
    "event": "message_stream",
    "id": "cm_12345",
    "message_id": "msg_67890",
    "answer": "Partial response chunk",
    "created_at": 1703624401
}
```

### 3. Message End Event
```json
{
    "event": "message_end",
    "id": "cm_12345",
    "message_id": "msg_67890",
    "answer": "The complete generated response text",
    "metadata": {
        "usage": {
            "prompt_tokens": 124,
            "completion_tokens": 356,
            "total_tokens": 480
        },
        "finish_reason": "stop",
        "model": "gpt-4",
        "generated_at": "2024-12-26T18:00:00Z"
    },
    "created_at": 1703624405
}
```

## Error Events

### Rate Limit Error
```json
{
    "event": "error",
    "error": {
        "message": "Rate limit exceeded",
        "type": "rate_limit_error",
        "code": "429",
        "details": {
            "retry_after": 60
        }
    }
}
```

### Model Error
```json
{
    "event": "error",
    "error": {
        "message": "Model temporarily unavailable",
        "type": "model_error",
        "code": "503",
        "details": {
            "model": "gpt-4",
            "retry_after": 300
        }
    }
}
```

### Validation Error
```json
{
    "event": "error",
    "error": {
        "message": "Invalid input parameters",
        "type": "validation_error",
        "code": "400",
        "details": {
            "field": "inputs.text",
            "reason": "Text input required"
        }
    }
}
```

## Processing Considerations

- Each streaming chunk starts with `data: ` and ends with `\n\n`
- Message IDs should be stored for debugging and tracking
- Token usage is only available in end events
- Error events can occur at any point in the stream
- Implement timeout handling for long-running completions
- Consider implementing retry logic for recoverable errors
- Monitor token usage for quota management

## Metadata Fields

| Field | Description | Available In |
|-------|-------------|--------------|
| usage | Token counts | End event |
| finish_reason | Completion status | End event |
| model | Model used | All events |
| generated_at | Timestamp | End event |

## File Handling

## File Handling Methods

Files can be included in completion message requests through multiple approaches:

### Direct File Upload
First upload the file using the `/files/upload` endpoint:
```json
{
    "fileUploadResponse": {
        "id": "file_id_12345",
        "name": "document.pdf",
        "url": "internal_file_reference"
    }
}
```

Then reference in completion request:
```json
{
    "inputs": {
        "text": "Analyze this document"
    },
    "files": [{
        "type": "document",
        "transfer_method": "local_file", 
        "url": "file_id_12345"
    }]
}
```

### Remote URL Method
```json
{
    "inputs": {
        "text": "Analyze this file"
    },
    "files": [{
        "type": "document",
        "transfer_method": "remote_url",
        "url": "https://example.com/document.pdf"
    }]
}
```

## File Specifications

**Size Limits:**
- Documents: 15MB maximum[6]
- Batch uploads: Maximum 20 files per request[6]

**Supported File Types:**
- Documents: PDF, DOCX, TXT[5]
- Images: Standard web formats
- Other file types require specific processing configurations

## Processing Considerations

- Files are automatically segmented and cleaned during upload[5]
- Two indexing modes available: high quality and economical[5]
- Files must be uploaded and processed before being used in completions
- Batch operations are supported for efficient handling of multiple files[2]
- Each file requires proper type specification for correct processing

Citations:
[1] https://pkg.go.dev/github.com/soulteary/dify-go-sdk
[2] https://docs.dify.ai/guides/knowledge-base/maintain-dataset-via-api
[3] https://docs.dify.ai/guides/application-publishing/developing-with-apis
[4] https://pkg.go.dev/github.com/kervinchang/dify-go
[5] https://www.restack.io/p/dify-answer-upload-file-cat-ai
[6] https://www.restack.io/p/dify-answer-digital-upload-cat-ai
[7] https://github.com/langgenius/dify/blob/main/api/controllers/console/app/completion.py

