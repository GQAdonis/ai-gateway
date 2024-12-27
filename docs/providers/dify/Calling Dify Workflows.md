# Overview

Dify workflows are called using the following example request:

```json
{
  "inputs": {
    "query":"Identify the food and create a recipe.",
    "foodImage": {
      "transfer_method":"remote_url",
      "type":"image",
      "url":"https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png"
    }
  },
  "response_mode":"streaming",
  "user":"abc"
}
```

This workflow has 2 inputs defined:

- `query`: Required query or instruction string
- `foodImage`: Optional image of food to analyze to determine what type of food is in the picture and perform the action that the query specifies based on what is found

## Dify Workflows API Spec

The following is the spec for calling Dify workflows via API using `curl` at the `/workflows/run` path off the base URL of `https://dify-api.prometheus-platform.io/v1`:

```bash
curl -X POST 'https://dify-api.prometheus-platform.io/v1/workflows/run' \
--header 'Authorization: Bearer {api_key}' \
--header 'Content-Type: application/json' \
--data-raw '{
    "inputs": {},
    "response_mode": "streaming",
    "user": "abc-123"
}'

```

## Requests

## API Specification

To execute a Dify workflow via the API endpoint `workflows/run`, you need to make a POST request with specific parameters and authentication.

## Authentication
Include an API key in the request header:
```http
Authorization: Bearer YOUR-API-KEY
Content-Type: application/json
```

## Request Format
```bash
curl -X POST 'http://your-host/v1/workflows/run' \
-H 'Authorization: Bearer YOUR-API-KEY' \
-H 'Content-Type: application/json' \
-d '{
    "inputs": {},
    "response_mode": "streaming",
    "blocking": true,
    "user": "user-id"
}'
```

## Required Parameters

**Request Body Parameters:**
- `inputs`: Object containing input variables for the workflow[1]
- `response_mode`: String specifying how responses should be delivered ("streaming" or non-streaming)[1]
- `blocking`: Boolean indicating whether to wait for workflow completion[1]
- `user`: String identifier for the user executing the workflow[1]

## System Variables

The following system variables are automatically assigned:

- `sys.app_id`: Unique identifier for the application[4]
- `sys.workflow_id`: Identifier for tracking node information[4]
- `sys.workflow_run_id`: Used for tracking execution logs and runtime status[4]
- `sys.user_id`: Unique identifier assigned to each user[4]
- `sys.files`: Array containing uploaded files (if enabled)[4]

## Response Handling

When executing a workflow:
- For non-blocking calls: You'll receive a `task_id` or `workflow_run_id` for later status checking[3]
- For streaming responses: Process the response stream as it arrives[3]
- For blocking calls: Wait for the complete workflow execution result[1]

Citations:
[1] https://github.com/langgenius/dify/issues/9513
[2] https://docs.dify.ai/guides/workflow/key-concepts
[3] https://github.com/langgenius/dify/issues/6350
[4] https://docs.dify.ai/guides/workflow/variables
[5] https://docs.dify.ai/guides/workflow/publish
[6] https://www.restack.io/p/dify-answer-workflow-example-cat-ai
[7] https://community.alteryx.com/t5/Alteryx-Designer-Desktop-Discussions/Run-workflow-by-making-API-call/td-p/1340658
[8] https://docs.dify.ai/guides/application-publishing/developing-with-apis

## Response Specifications

### Blocking Mode Response

When using `response_mode: "blocking"`, you'll receive a single JSON response:

```json
{
    "id": "unique-thought-id",
    "message_id": "unique-message-id",
    "conversation_id": "session-id",
    "mode": "chat",
    "answer": "Complete response content",
    "metadata": {
        // Additional metadata
    },
    "created_at": 1705395332
}
```

### Streaming Mode Events

When using `response_mode: "streaming"`, you'll receive a series of events in the following order:

**1. Message Start Event**
```json
{
    "event": "message_start",
    "id": "thought-id",
    "message_id": "message-id",
    "conversation_id": "session-id",
    "created_at": 1705395332
}
```

**2. Message Stream Event**
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

**3. Message End Event**
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

**4. Workflow Finished Event** (Only for workflow calls)
```json
{
    "event": "workflow_finished",
    "workflow_run_id": "workflow-execution-id",
    "status": "succeeded",
    "outputs": {
        // Workflow outputs
    },
    "elapsed_time": 2.5,
    "total_tokens": 150,
    "total_steps": 3
}
```

## Error Response

For both modes, error responses follow this format:
```json
{
    "error": {
        "message": "Error description",
        "type": "error_type",
        "code": "error_code"
    }
}
```

Citations:
[1] https://www.restack.io/p/dify-add-option-for-blocking-in-api-call-to-chat-agent
[2] https://pkg.go.dev/github.com/kervinchang/dify-go
[3] https://docs.dify.ai/guides/application-publishing/developing-with-apis
[4] https://www.restack.io/p/dify-answer-open-api-examples-cat-ai
[5] https://github.com/langgenius/dify/discussions/6477
[6] https://github.com/langgenius/dify/issues/9513
[7] https://github.com/langgenius/dify/issues/11477
[8] https://www.restack.io/p/dify-json-schema-support-in-dify-0-9-1
[9] https://github.com/langgenius/dify/discussions/8599
[10] https://dev.to/ku6ryo/how-to-realize-real-time-speech-with-dify-api-4ii1

## Example Streaming Response

The following is the example streaming response from our sample request:

```
data: {"event": "workflow_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "workflow_id": "2b208495-b5a4-401a-a3f8-6e772ef26ab7", "sequence_number": 14, "inputs": {"foodImage": {"dify_model_identity": "__dify__file__", "id": null, "tenant_id": "237e28c3-c6a8-4c2a-99ac-59beb4826f77", "type": "image", "transfer_method": "remote_url", "remote_url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png", "related_id": null, "filename": "quinoia_bean_bowl.png", "extension": ".png", "mime_type": "image/png", "size": 1668721, "url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png"}, "query": "Identify the food and create a recipe.", "sys.files": [], "sys.user_id": "abc", "sys.app_id": "c6b7dbbd-6bfa-4e2a-9fc3-ff6e153695e0", "sys.workflow_id": "2b208495-b5a4-401a-a3f8-6e772ef26ab7", "sys.workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa"}, "created_at": 1735256004}}

data: {"event": "node_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "a7726f9d-d798-4803-bec9-a25c25a17046", "node_id": "1734941806037", "node_type": "start", "title": "Start", "index": 1, "predecessor_node_id": null, "inputs": null, "created_at": 1735256004, "extras": {}, "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null, "parallel_run_id": null}}

data: {"event": "node_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "a7726f9d-d798-4803-bec9-a25c25a17046", "node_id": "1734941806037", "node_type": "start", "title": "Start", "index": 1, "predecessor_node_id": null, "inputs": {"foodImage": {"dify_model_identity": "__dify__file__", "id": null, "tenant_id": "237e28c3-c6a8-4c2a-99ac-59beb4826f77", "type": "image", "transfer_method": "remote_url", "remote_url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png", "related_id": null, "filename": "quinoia_bean_bowl.png", "extension": ".png", "mime_type": "image/png", "size": 1668721, "url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png"}, "query": "Identify the food and create a recipe.", "sys.files": [], "sys.user_id": "abc", "sys.app_id": "c6b7dbbd-6bfa-4e2a-9fc3-ff6e153695e0", "sys.workflow_id": "2b208495-b5a4-401a-a3f8-6e772ef26ab7", "sys.workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa"}, "process_data": null, "outputs": {"foodImage": {"dify_model_identity": "__dify__file__", "id": null, "tenant_id": "237e28c3-c6a8-4c2a-99ac-59beb4826f77", "type": "image", "transfer_method": "remote_url", "remote_url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png", "related_id": null, "filename": "quinoia_bean_bowl.png", "extension": ".png", "mime_type": "image/png", "size": 1668721, "url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png"}, "query": "Identify the food and create a recipe.", "sys.files": [], "sys.user_id": "abc", "sys.app_id": "c6b7dbbd-6bfa-4e2a-9fc3-ff6e153695e0", "sys.workflow_id": "2b208495-b5a4-401a-a3f8-6e772ef26ab7", "sys.workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa"}, "status": "succeeded", "error": null, "elapsed_time": 0.041375, "execution_metadata": null, "created_at": 1735256004, "finished_at": 1735256004, "files": [{"dify_model_identity": "__dify__file__", "id": null, "tenant_id": "237e28c3-c6a8-4c2a-99ac-59beb4826f77", "type": "image", "transfer_method": "remote_url", "remote_url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png", "related_id": null, "filename": "quinoia_bean_bowl.png", "extension": ".png", "mime_type": "image/png", "size": 1668721, "url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png"}], "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null}}

data: {"event": "node_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "a10e3b01-bb8c-4f04-aa39-1463bf1c96aa", "node_id": "1734945093805", "node_type": "llm", "title": "LLM 2", "index": 2, "predecessor_node_id": "1734941806037", "inputs": null, "created_at": 1735256004, "extras": {}, "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null, "parallel_run_id": null}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "The", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " image", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " showcases", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " a", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " vibrant", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " nutritious", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " bowl", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " of", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " food", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " likely", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " designed", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " to", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " be", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " both", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " visually", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " appealing", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " healthy", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Key", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Components", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "**\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Base", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " A", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " light", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " blue", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ceramic", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " bowl", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " with", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " brown", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " spe", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ck", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "les", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " around", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " its", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " rim", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " holds", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " main", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ingredients", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Gr", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ain", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Component", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Yellow", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " grains", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " possibly", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " millet", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " or", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cous", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "c", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ous", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " fill", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " most", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " of", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " bowl", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Pro", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "tein", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "-R", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ich", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Element", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Black", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " beans", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " are", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " scattered", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " on", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " top", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " of", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " grains", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " adding", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " protein", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " fiber", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " to", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " dish", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "D", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "airy", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Component", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Cr", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "umb", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ly", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " white", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cheese", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " is", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " sprink", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "led", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " over", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " black", "from_variable_selector": ["1734945093805", "text"]}}

event: ping

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " beans", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " providing", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " a", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " tang", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "y", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " flavor", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Ve", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "get", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "able", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Component", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Ch", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "opped", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " green", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " onions", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " garn", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ish", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " dish", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " contributing", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " freshness", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " crunch", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Recipe", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "**\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "To", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " create", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " this", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " recipe", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " gather", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " following", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ingredients", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "1", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cup", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " yellow", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " grains", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " (", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "m", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "illet", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " or", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cous", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "c", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ous", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ")\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "1", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " can", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " black", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " beans", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "1", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "/", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "2", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cup", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cr", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "umbled", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " white", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cheese", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "*", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " ", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "1", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "/", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "4", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cup", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " chopped", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " green", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " onions", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Instructions", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "**\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "1", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Prepare", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Gr", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "ains", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Cook", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " yellow", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " grains", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " according", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " to", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " package", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " instructions", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "2", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Prepare", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Black", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Beans", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Drain", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " rinse", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " canned", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " black", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " beans", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "3", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "As", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "semble", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " the", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Dish", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Place", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cooked", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " grains", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " in", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " a", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " bowl", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " top", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " with", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " black", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " beans", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cr", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "umbled", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " cheese", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " chopped", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " green", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " onions", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "4", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " **", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "Serve", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Enjoy", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ":**", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " Serve", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " immediately", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " enjoy", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "!\n\n", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "This", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " recipe", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " offers", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " a", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " balanced", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " mix", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " of", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " complex", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " carbohydrates", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " protein", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " and", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " healthy", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " fats", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ",", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " making", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " it", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " an", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " excellent", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " option", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " for", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " those", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " seeking", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " a", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " nutritious", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": " meal", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": ".", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "text_chunk", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"text": "", "from_variable_selector": ["1734945093805", "text"]}}

data: {"event": "node_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "a10e3b01-bb8c-4f04-aa39-1463bf1c96aa", "node_id": "1734945093805", "node_type": "llm", "title": "LLM 2", "index": 2, "predecessor_node_id": "1734941806037", "inputs": {"#files#": [{"dify_model_identity": "__dify__file__", "id": null, "tenant_id": "237e28c3-c6a8-4c2a-99ac-59beb4826f77", "type": "image", "transfer_method": "remote_url", "remote_url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png", "related_id": null, "filename": "quinoia_bean_bowl.png", "extension": ".png", "mime_type": "image/png", "size": 1668721, "url": "https://supabase.prometheus-platform.io/storage/v1/object/public/foodonthestove/quinoia_bean_bowl.png"}]}, "process_data": {"model_mode": "chat", "prompts": [{"role": "user", "text": "Identify the food and create a recipe.", "files": [{"type": "image", "data": "data:image...[TRUNCATED]...5ErkJggg==", "detail": "high"}]}], "model_provider": "ollama", "model_name": "llama3.2-vision:latest"}, "outputs": {"text": "The image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal.", "usage": {"prompt_tokens": 21, "prompt_unit_price": "0", "prompt_price_unit": "0", "prompt_price": "0E-7", "completion_tokens": 305, "completion_unit_price": "0", "completion_price_unit": "0", "completion_price": "0E-7", "total_tokens": 326, "total_price": "0E-7", "currency": "USD", "latency": 14.185634054010734}, "finish_reason": "stop"}, "status": "succeeded", "error": null, "elapsed_time": 14.348465, "execution_metadata": {"total_tokens": 326, "total_price": "0.0000000", "currency": "USD"}, "created_at": 1735256004, "finished_at": 1735256019, "files": [], "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null}}

data: {"event": "node_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "017dad17-5686-476c-9183-5146362a1422", "node_id": "1734959046437", "node_type": "template-transform", "title": "Template", "index": 3, "predecessor_node_id": "1734945093805", "inputs": null, "created_at": 1735256019, "extras": {}, "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null, "parallel_run_id": null}}

data: {"event": "node_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "017dad17-5686-476c-9183-5146362a1422", "node_id": "1734959046437", "node_type": "template-transform", "title": "Template", "index": 3, "predecessor_node_id": "1734945093805", "inputs": {"imageDescription": "The image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal.", "query": "Identify the food and create a recipe."}, "process_data": null, "outputs": {"output": "<instructions>\nIdentify the food and create a recipe.\n</instructions>\n\n<description>\nThe image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal.\n</description>"}, "status": "succeeded", "error": null, "elapsed_time": 0.141691, "execution_metadata": null, "created_at": 1735256019, "finished_at": 1735256019, "files": [], "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null}}

data: {"event": "node_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "a0c2bedb-b645-4776-a3e3-dfb7ddfb8724", "node_id": "1734958923467", "node_type": "knowledge-retrieval", "title": "Knowledge Retrieval", "index": 4, "predecessor_node_id": "1734959046437", "inputs": null, "created_at": 1735256019, "extras": {}, "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null, "parallel_run_id": null}}

data: {"event": "node_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "a0c2bedb-b645-4776-a3e3-dfb7ddfb8724", "node_id": "1734958923467", "node_type": "knowledge-retrieval", "title": "Knowledge Retrieval", "index": 4, "predecessor_node_id": "1734959046437", "inputs": {"query": "Identify the food and create a recipe."}, "process_data": null, "outputs": {"result": []}, "status": "succeeded", "error": null, "elapsed_time": 0.054811, "execution_metadata": null, "created_at": 1735256019, "finished_at": 1735256019, "files": [], "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null}}

data: {"event": "node_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "0c8a8b54-f3f7-4708-b8c6-dfe427d5987f", "node_id": "1734959736825", "node_type": "llm", "title": "LLM 2", "index": 5, "predecessor_node_id": "1734958923467", "inputs": null, "created_at": 1735256019, "extras": {}, "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null, "parallel_run_id": null}}

data: {"event": "node_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "0c8a8b54-f3f7-4708-b8c6-dfe427d5987f", "node_id": "1734959736825", "node_type": "llm", "title": "LLM 2", "index": 5, "predecessor_node_id": "1734958923467", "inputs": null, "process_data": {"model_mode": "chat", "prompts": [{"role": "system", "text": "Utilize the matching schema context to create JSON output matching one or more schemas found:\n\n{{#context#}}\n\nOnly provide answers as JSON matching a schema in the knowledge base.", "files": []}, {"role": "user", "text": "<instructions>\nIdentify the food and create a recipe.\n</instructions>\n\n<description>\nThe image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal.\n</description>", "files": []}], "model_provider": "azure_openai", "model_name": "gpt-4o"}, "outputs": {"text": "```json\n{\n  \"recipe\": {\n    \"name\": \"Vibrant Grain Bowl\",\n    \"ingredients\": [\n      {\n        \"ingredient\": \"yellow grains (millet or couscous)\",\n        \"quantity\": \"1 cup\"\n      },\n      {\n        \"ingredient\": \"black beans\",\n        \"quantity\": \"1 can\"\n      },\n      {\n        \"ingredient\": \"crumbled white cheese\",\n        \"quantity\": \"1/2 cup\"\n      },\n      {\n        \"ingredient\": \"chopped green onions\",\n        \"quantity\": \"1/4 cup\"\n      }\n    ],\n    \"instructions\": [\n      \"Cook the yellow grains according to package instructions.\",\n      \"Drain and rinse the canned black beans.\",\n      \"Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\",\n      \"Serve immediately and enjoy!\"\n    ],\n    \"description\": \"A vibrant and nutritious bowl featuring yellow grains, black beans, crumbled white cheese, and chopped green onions. This dish offers a balanced mix of complex carbohydrates, protein, and healthy fats.\"\n  }\n}\n```", "usage": {"prompt_tokens": 365, "prompt_unit_price": "5.0", "prompt_price_unit": "0.000001", "prompt_price": "0.0018250", "completion_tokens": 234, "completion_unit_price": "15.0", "completion_price_unit": "0.000001", "completion_price": "0.0035100", "total_tokens": 599, "total_price": "0.0053350", "currency": "USD", "latency": 3.810139387031086}, "finish_reason": "stop"}, "status": "succeeded", "error": null, "elapsed_time": 3.89753, "execution_metadata": {"total_tokens": 599, "total_price": "0.0053350", "currency": "USD"}, "created_at": 1735256019, "finished_at": 1735256023, "files": [], "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null}}

data: {"event": "node_started", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "11575a5b-6008-449c-8bb4-a81712290e8f", "node_id": "1734942605284", "node_type": "end", "title": "End", "index": 6, "predecessor_node_id": "1734959736825", "inputs": null, "created_at": 1735256023, "extras": {}, "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null, "parallel_run_id": null}}

data: {"event": "node_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "11575a5b-6008-449c-8bb4-a81712290e8f", "node_id": "1734942605284", "node_type": "end", "title": "End", "index": 6, "predecessor_node_id": "1734959736825", "inputs": {"text": "The image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal."}, "process_data": null, "outputs": {"text": "The image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal."}, "status": "succeeded", "error": null, "elapsed_time": 0.040209, "execution_metadata": null, "created_at": 1735256023, "finished_at": 1735256023, "files": [], "parallel_id": null, "parallel_start_node_id": null, "parent_parallel_id": null, "parent_parallel_start_node_id": null, "iteration_id": null}}

data: {"event": "workflow_finished", "workflow_run_id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "task_id": "81156813-ed3f-44d6-afde-db207fb1cc3b", "data": {"id": "b958d7f1-f6b9-45da-91e6-650979d218fa", "workflow_id": "2b208495-b5a4-401a-a3f8-6e772ef26ab7", "sequence_number": 14, "status": "succeeded", "outputs": {"text": "The image showcases a vibrant and nutritious bowl of food, likely designed to be both visually appealing and healthy.\n\n**Key Components:**\n\n* **Base:** A light blue ceramic bowl with brown speckles around its rim holds the main ingredients.\n* **Grain Component:** Yellow grains, possibly millet or couscous, fill most of the bowl.\n* **Protein-Rich Element:** Black beans are scattered on top of the grains, adding protein and fiber to the dish.\n* **Dairy Component:** Crumbly white cheese is sprinkled over the black beans, providing a tangy flavor.\n* **Vegetable Component:** Chopped green onions garnish the dish, contributing freshness and crunch.\n\n**Recipe:**\n\nTo create this recipe, gather the following ingredients:\n\n* 1 cup yellow grains (millet or couscous)\n* 1 can black beans\n* 1/2 cup crumbled white cheese\n* 1/4 cup chopped green onions\n\n**Instructions:**\n\n1. **Prepare the Grains:** Cook the yellow grains according to package instructions.\n2. **Prepare the Black Beans:** Drain and rinse the canned black beans.\n3. **Assemble the Dish:** Place cooked grains in a bowl, top with black beans, crumbled cheese, and chopped green onions.\n4. **Serve and Enjoy:** Serve immediately and enjoy!\n\nThis recipe offers a balanced mix of complex carbohydrates, protein, and healthy fats, making it an excellent option for those seeking a nutritious meal."}, "error": null, "elapsed_time": 18.73071221797727, "total_tokens": 925, "total_steps": 6, "created_by": {"id": "fe03310b-9a4b-4b42-a9d1-334884a2d077", "user": "abc"}, "created_at": 1735256004, "finished_at": 1735256023, "exceptions_count": 0, "files": []}}


```

## Blocking Response

The following is an example blocking response from our sample request:

```json
{
  "task_id": "3e06bf66-d6f3-4a4d-a9ba-9ce5d15a53ff",
  "workflow_run_id": "f5a310df-508e-4a06-a7f8-92633c192c6b",
  "data": {
    "id": "f5a310df-508e-4a06-a7f8-92633c192c6b",
    "workflow_id": "2b208495-b5a4-401a-a3f8-6e772ef26ab7",
    "status": "succeeded",
    "outputs": {
      "text": "**Black Bean Bowl with Avocado Crema**\n\nThis vibrant bowl features a harmonious blend of flavors, textures, and colors. Here's how to recreate it:\n\n**Ingredients:**\n\n• 1 cup cooked black beans\n• 1/2 cup quinoa or couscous\n• 1/4 cup diced red onion\n• 1/4 cup chopped fresh cilantro\n• 1 tablespoon olive oil\n• Salt and pepper to taste\n• Optional toppings: crumbled feta cheese, sliced avocado, lime wedges\n\n**Instructions:**\n\n1. Cook quinoa or couscous according to package instructions.\n2. In a large bowl, combine cooked black beans, red onion, and cilantro.\n3. Drizzle olive oil over the mixture and season with salt and pepper.\n4. Serve black bean and quinoa mixture in bowls.\n5. Optional: Garnish with crumbled feta cheese, sliced avocado, or lime wedges.\n\n**Avocado Crema**\n\n• 1 ripe avocado\n• 1/2 cup Greek yogurt\n• Juice of 1 lime\n• Salt and pepper to taste\n\nInstructions:\n\n1. Peel and pit the avocado.\n2. In a blender or food processor, combine avocado, Greek yogurt, lime juice, salt, and pepper.\n3. Blend until smooth and creamy.\n\n**Tips:**\n\n• For extra flavor, add diced jalapeño or serrano peppers to the black bean mixture.\n• Substitute other types of beans, such as kidney or pinto beans, for the black beans.\n• Experiment with different seasonings, like cumin or smoked paprika, to give the dish a unique twist.\n\nThis recipe is perfect for a quick and nutritious lunch or dinner. The combination of protein-rich black beans, complex carbohydrates from quinoa or couscous, and healthy fats from avocado crema provides a balanced meal that's both delicious and satisfying."
    },
    "error": null,
    "elapsed_time": 34.66956109192688,
    "total_tokens": 1406,
    "total_steps": 6,
    "created_at": 1735257262,
    "finished_at": 1735257296
  }
}
```

## Images and Other File Types as Input

## File Input Methods

Files can be passed to Dify workflows in three ways: URLs, base64 encoding, or file reference IDs.

## URL Method
```javascript
{
    "inputs": {
        "document_input": {
            "type": "document",
            "value": {
                "url": "https://example.com/document.pdf",
                "name": "document.pdf"
            }
        }
    }
}
```

## Base64 Method
```javascript
{
    "inputs": {
        "image_input": {
            "type": "image",
            "value": {
                "data": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEAYABgAAD...",
                "name": "image.jpg"
            }
        }
    }
}
```

## File Reference Method
```javascript
{
    "inputs": {
        "document_input": {
            "type": "document",
            "value": {
                "url": "file_id_12345",
                "name": "uploaded_doc.pdf"
            }
        }
    }
}
```

## Multiple Files Using Mixed Methods
```javascript
{
    "inputs": {
        "mixed_files": {
            "type": "array[file]",
            "value": [
                {
                    "url": "https://example.com/remote_file.pdf",
                    "name": "remote_file.pdf"
                },
                {
                    "data": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEAYABgAAD...",
                    "name": "base64_image.jpg"
                },
                {
                    "url": "file_id_67890",
                    "name": "uploaded_file.docx"
                }
            ]
        }
    }
}
```

## File Type Support Matrix

| File Type | URL | Base64 | File Reference |
|-----------|-----|--------|----------------|
| Document  | ✓   | ✓      | ✓             |
| Image     | ✓   | ✓      | ✓             |
| Audio     | ✓   | ✓      | ✓             |
| Video     | ✓   | ✓      | ✓             |
| Other     | ✓   | ✓      | ✓             |

## Important Considerations

- Base64 encoded files must include the appropriate data URI prefix (e.g., `data:image/jpeg;base64,`)
- File size limits apply regardless of input method:
  - Images: 10MB
  - Documents: 50MB
  - Audio: 20MB
  - Video: 100MB
- URLs must be publicly accessible
- File references must be obtained through prior upload API calls
- All methods support the same file type processing capabilities in the workflow

