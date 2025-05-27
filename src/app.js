const express = require('express');
const cors = require('cors');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;

// ミドルウェア
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(express.static(path.join(__dirname, 'public')));

// ユーザーデータ（メモリ内ストレージ）
let users = [
  { id: 1, name: '田中太郎', email: 'tanaka@example.com', age: 30 },
  { id: 2, name: '佐藤花子', email: 'sato@example.com', age: 25 }
];

let todos = [
  { id: 1, title: '買い物に行く', completed: false, userId: 1 },
  { id: 2, title: 'レポートを書く', completed: true, userId: 1 },
  { id: 3, title: '会議の準備', completed: false, userId: 2 }
];

// ルート
app.get('/', (req, res) => {
  res.send(`
    <!DOCTYPE html>
    <html lang="ja">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>BDD E2E テストサンプルアプリ</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .container { max-width: 800px; margin: 0 auto; }
            .section { margin: 20px 0; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }
            button { padding: 10px 15px; margin: 5px; cursor: pointer; }
            input, select { padding: 8px; margin: 5px; }
            .todo-item { padding: 10px; margin: 5px 0; border: 1px solid #eee; }
            .completed { text-decoration: line-through; color: #888; }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>BDD E2E テストサンプルアプリ</h1>
            
            <div class="section">
                <h2>ユーザー管理</h2>
                <div id="user-form">
                    <input type="text" id="user-name" placeholder="名前" />
                    <input type="email" id="user-email" placeholder="メールアドレス" />
                    <input type="number" id="user-age" placeholder="年齢" />
                    <button onclick="addUser()">ユーザー追加</button>
                </div>
                <div id="users-list"></div>
            </div>
            
            <div class="section">
                <h2>TODO管理</h2>
                <div id="todo-form">
                    <input type="text" id="todo-title" placeholder="TODOタイトル" />
                    <select id="todo-user">
                        <option value="">ユーザーを選択</option>
                    </select>
                    <button onclick="addTodo()">TODO追加</button>
                </div>
                <div id="todos-list"></div>
            </div>
        </div>

        <script>
            // ユーザー一覧を取得・表示
            async function loadUsers() {
                const response = await fetch('/api/users');
                const users = await response.json();
                const usersList = document.getElementById('users-list');
                const userSelect = document.getElementById('todo-user');
                
                usersList.innerHTML = users.map(user => 
                    \`<div class="user-item" data-user-id="\${user.id}">
                        <strong>\${user.name}</strong> (\${user.email}) - \${user.age}歳
                        <button onclick="deleteUser(\${user.id})">削除</button>
                    </div>\`
                ).join('');
                
                userSelect.innerHTML = '<option value="">ユーザーを選択</option>' + 
                    users.map(user => \`<option value="\${user.id}">\${user.name}</option>\`).join('');
            }

            // TODO一覧を取得・表示
            async function loadTodos() {
                const response = await fetch('/api/todos');
                const todos = await response.json();
                const todosList = document.getElementById('todos-list');
                
                todosList.innerHTML = todos.map(todo => 
                    \`<div class="todo-item \${todo.completed ? 'completed' : ''}" data-todo-id="\${todo.id}">
                        <input type="checkbox" \${todo.completed ? 'checked' : ''} 
                               onchange="toggleTodo(\${todo.id})" />
                        <span>\${todo.title}</span>
                        <button onclick="deleteTodo(\${todo.id})">削除</button>
                    </div>\`
                ).join('');
            }

            // ユーザー追加
            async function addUser() {
                const name = document.getElementById('user-name').value;
                const email = document.getElementById('user-email').value;
                const age = document.getElementById('user-age').value;
                
                if (!name || !email || !age) {
                    alert('すべての項目を入力してください');
                    return;
                }
                
                await fetch('/api/users', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name, email, age: parseInt(age) })
                });
                
                document.getElementById('user-name').value = '';
                document.getElementById('user-email').value = '';
                document.getElementById('user-age').value = '';
                loadUsers();
            }

            // TODO追加
            async function addTodo() {
                const title = document.getElementById('todo-title').value;
                const userId = document.getElementById('todo-user').value;
                
                if (!title || !userId) {
                    alert('タイトルとユーザーを選択してください');
                    return;
                }
                
                await fetch('/api/todos', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ title, userId: parseInt(userId) })
                });
                
                document.getElementById('todo-title').value = '';
                loadTodos();
            }

            // TODO完了状態切り替え
            async function toggleTodo(id) {
                await fetch(\`/api/todos/\${id}/toggle\`, { method: 'PUT' });
                loadTodos();
            }

            // ユーザー削除
            async function deleteUser(id) {
                if (confirm('このユーザーを削除しますか？')) {
                    await fetch(\`/api/users/\${id}\`, { method: 'DELETE' });
                    loadUsers();
                    loadTodos();
                }
            }

            // TODO削除
            async function deleteTodo(id) {
                if (confirm('このTODOを削除しますか？')) {
                    await fetch(\`/api/todos/\${id}\`, { method: 'DELETE' });
                    loadTodos();
                }
            }

            // 初期化
            loadUsers();
            loadTodos();
        </script>
    </body>
    </html>
  `);
});

// API エンドポイント

// ユーザー関連API
app.get('/api/users', (req, res) => {
  res.json(users);
});

app.post('/api/users', (req, res) => {
  const { name, email, age } = req.body;
  const newUser = {
    id: Math.max(...users.map(u => u.id), 0) + 1,
    name,
    email,
    age
  };
  users.push(newUser);
  res.status(201).json(newUser);
});

app.delete('/api/users/:id', (req, res) => {
  const id = parseInt(req.params.id);
  users = users.filter(u => u.id !== id);
  todos = todos.filter(t => t.userId !== id);
  res.status(204).send();
});

// TODO関連API
app.get('/api/todos', (req, res) => {
  res.json(todos);
});

app.post('/api/todos', (req, res) => {
  const { title, userId } = req.body;
  const newTodo = {
    id: Math.max(...todos.map(t => t.id), 0) + 1,
    title,
    completed: false,
    userId
  };
  todos.push(newTodo);
  res.status(201).json(newTodo);
});

app.put('/api/todos/:id/toggle', (req, res) => {
  const id = parseInt(req.params.id);
  const todo = todos.find(t => t.id === id);
  if (todo) {
    todo.completed = !todo.completed;
    res.json(todo);
  } else {
    res.status(404).json({ error: 'Todo not found' });
  }
});

app.delete('/api/todos/:id', (req, res) => {
  const id = parseInt(req.params.id);
  todos = todos.filter(t => t.id !== id);
  res.status(204).send();
});

// サーバー起動
if (require.main === module) {
  app.listen(PORT, () => {
    console.log(`サーバーがポート ${PORT} で起動しました`);
    console.log(`http://localhost:${PORT} でアクセスできます`);
  });
}

module.exports = app;