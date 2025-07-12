# Interactive Mode Examples

This guide walks you through using Terzi's interactive mode for guided API request building.

## 🚀 Getting Started

Launch interactive mode:

```bash
terzi interactive
```

You'll see a welcome message and the main menu.

## 📋 Main Menu Options

When you start interactive mode, you'll see these options:

```
🚀 Create New Request
📋 Load Saved Request
📚 Browse Request Collection
🔍 Search History
⚙️  Settings
🚪 Exit
```

## 🛠️ Creating a New Request

### Step 1: Select "Create New Request"

Choose the first option to start building a new request.

### Step 2: Enter URL

You'll be prompted to enter a URL:

```
Enter the URL: https://jsonplaceholder.typicode.com/posts/1
```

The system validates that your URL starts with `http://` or `https://`.

### Step 3: Select HTTP Method

Choose from available methods:

```
Select HTTP method:
❯ GET
  POST
  PUT
  DELETE
  PATCH
  HEAD
  OPTIONS
```

### Step 4: Add Headers (Optional)

You'll be asked if you want to add custom headers:

```
Add custom headers? (y/N)
```

If you choose yes, you can add headers one by one:

```
Header name: Authorization
Value for 'Authorization': Bearer your-token-here

Add another header? (y/N)
```

### Step 5: Add Authentication (Optional)

You'll be asked if you want to add authentication:

```
Add authentication? (y/N)
```

If yes, you can choose from:

```
Select authentication type:
❯ Bearer Token
  Basic Auth
  API Key (Header)
  API Key (Query)
```

#### Bearer Token
```
Enter bearer token: your-token-here
```

#### Basic Auth
```
Username: your-username
Password: [hidden input]
```

#### API Key (Header)
```
Header name: X-API-Key
API key value: your-api-key
```

### Step 6: Add Request Body (for POST/PUT/PATCH)

For non-GET requests, you can add a body:

```
Add request body? (y/N)
```

Choose the body type:

```
Select body type:
❯ JSON
  Form Data
  Raw Text
  File Upload
```

#### JSON Body
```
JSON: {"title": "Test Post", "body": "This is a test", "userId": 1}
```

The system validates JSON syntax and shows success/error messages.

#### Form Data
```
Form field name: title
Value for 'title': Test Post

Form field name: body
Value for 'body': This is a test post

Add another field? (y/N)
```

#### Raw Text
```
Enter raw body content: This is raw text data
```

### Step 7: Preview and Execute

Before executing, you'll see a preview:

```
📋 Request Preview
URL:    https://jsonplaceholder.typicode.com/posts
Method: POST
Headers:
  Content-Type: application/json
  Authorization: Bearer your-token
Body:
  {"title": "Test Post", "body": "This is a test", "userId": 1}

Execute this request? (Y/n)
```

### Step 8: Save Request (Optional)

After a successful request, you can save it:

```
Save this request for future use? (y/N)

Enter a name for this request: create-test-post
```

## 📋 Loading Saved Requests

### Step 1: Select "Load Saved Request"

Choose the second option from the main menu.

### Step 2: Choose from Saved Requests

You'll see a fuzzy searchable list:

```
Select a request to load:
❯ create-test-post
  get-user-profile
  github-repos
  api-status-check
```

Use arrow keys or type to filter the list.

### Step 3: Preview and Execute

The request details will be shown, then you can execute it:

```
📋 Request Details: create-test-post
🔗 URL: https://jsonplaceholder.typicode.com/posts
🔧 Method: POST
📤 Headers:
  Content-Type: application/json
📝 Body: {"title": "Test Post", "body": "This is a test", "userId": 1}
📅 Created: 2024-01-15 14:30

Execute this request? (Y/n)
```

## 📚 Browse Request Collection

### Step 1: Select "Browse Request Collection"

Choose the third option to view all your saved requests.

### Step 2: View Request Table

You'll see a table of all saved requests:

```
┌───┬─────────────────┬────────┬──────────────────────────────────┬─────────────────────┐
│ # │ Name            │ Method │ URL                              │ Created             │
├───┼─────────────────┼────────┼──────────────────────────────────┼─────────────────────┤
│ 1 │ create-test-post│ POST   │ https://jsonplaceholder.typicode │ 2024-01-15 14:30    │
│ 2 │ get-user-profile│ GET    │ https://api.github.com/user      │ 2024-01-15 14:25    │
│ 3 │ github-repos    │ GET    │ https://api.github.com/user/repos│ 2024-01-15 14:20    │
└───┴─────────────────┴────────┴──────────────────────────────────┴─────────────────────┘
```

### Step 3: Choose Action

Select what you want to do:

```
What would you like to do?
❯ View Request Details
  Execute Request
  Edit Request
  Delete Request
  Back to Main Menu
```

### Step 4: Select Request

Choose a request from the list and perform the selected action.

## 🔍 Search History

### Step 1: Select "Search History"

Choose the fourth option to view your request history.

### Step 2: View History Table

You'll see a table of recent requests:

```
┌─────────┬────────┬─────────────────────────────────────┬────────────┬──────────┐
│ Time    │ Method │ URL                                 │ Status     │ Duration │
├─────────┼────────┼─────────────────────────────────────┼────────────┼──────────┤
│ 14:30:15│ POST   │ https://jsonplaceholder.typicode... │ 🟢 201     │ 156ms    │
│ 14:25:43│ GET    │ https://api.github.com/user         │ 🟢 200     │ 342ms    │
│ 14:20:12│ GET    │ https://api.github.com/user/repos   │ 🟢 200     │ 189ms    │
└─────────┴────────┴─────────────────────────────────────┴────────────┴──────────┘
```

This shows:
- ✅ **Time**: When the request was made
- ✅ **Method**: HTTP method used
- ✅ **URL**: Request URL (truncated for display)
- ✅ **Status**: Response status with color coding
- ✅ **Duration**: How long the request took

## ⚙️ Settings Menu

### Step 1: Select "Settings"

Choose the fifth option to access settings.

### Step 2: Choose Setting Action

```
Settings
❯ View Current Settings
  Change Default Timeout
  Change Output Format
  Reset to Defaults
  Back to Main Menu
```

### Step 3: Modify Settings

For example, changing the default timeout:

```
Enter new timeout value (seconds): 60
✅ Default timeout set to 60 seconds
```

## 🎯 Advanced Interactive Features

### Request Editing

When editing a saved request:

```
✏️  Editing Request: create-test-post

📋 Request Preview
URL:    https://jsonplaceholder.typicode.com/posts
Method: POST
Headers:
  Content-Type: application/json
Body:
  {"title": "Test Post", "body": "This is a test", "userId": 1}

What would you like to edit?
❯ Change URL
  Change Method
  Edit Headers
  Edit Body
  Save Changes
  Cancel
```

### Fuzzy Search

The interactive mode uses fuzzy search for:
- ✅ **Request names**: Type part of a name to filter
- ✅ **URL search**: Find requests by URL patterns
- ✅ **Method filtering**: Filter by HTTP method

### Validation and Error Handling

The interactive mode provides:
- ✅ **URL validation**: Ensures URLs are properly formatted
- ✅ **JSON validation**: Validates JSON syntax in real-time
- ✅ **Header validation**: Checks header name/value format
- ✅ **Method validation**: Ensures valid HTTP methods

## 💡 Interactive Mode Tips

### 1. Use Descriptive Names
```
Good: github-user-profile, api-create-user, auth-login
Bad: req1, test, api
```

### 2. Save Complex Requests
Save requests with:
- Multiple headers
- Authentication
- Complex JSON payloads
- Custom configurations

### 3. Use Preview Before Execution
Always review the request preview to ensure:
- URL is correct
- Headers are properly set
- Body content is valid
- Authentication is included

### 4. Organize Your Collection
Use consistent naming:
- `github-*` for GitHub API requests
- `auth-*` for authentication requests
- `api-*` for general API requests

### 5. Regular Cleanup
Periodically:
- Delete unused requests
- Update outdated requests
- Export collections for backup

## 🚀 Workflow Examples

### API Development Workflow

1. **Create and test** new endpoint
2. **Save successful** request
3. **Load and modify** for different test cases
4. **Export collection** for team sharing

### API Testing Workflow

1. **Load base request** from collection
2. **Edit parameters** for specific test
3. **Execute and verify** response
4. **Save variations** for regression testing

### API Documentation Workflow

1. **Create requests** for all endpoints
2. **Add descriptions** and examples
3. **Organize by feature** or service
4. **Export for documentation**

## 🎉 Summary

Interactive mode provides:
- ✅ **Guided request building** with step-by-step prompts
- ✅ **Visual feedback** with colors and formatting
- ✅ **Error handling** with helpful messages
- ✅ **Request management** with save/load/edit
- ✅ **History tracking** for all requests
- ✅ **Fuzzy search** for quick navigation

Perfect for:
- 🎯 **Learning** API endpoints
- 🎯 **Building** complex requests
- 🎯 **Managing** request collections
- 🎯 **Debugging** API issues
- 🎯 **Sharing** API workflows

## 📖 Next Steps

After mastering interactive mode:

1. Try the [command-line examples](basic-requests.sh)
2. Learn about [authentication](authentication.sh)
3. Explore [configuration options](configuration.sh)
4. Check out [advanced features](advanced-usage.sh) 