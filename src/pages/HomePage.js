class HomePage {
  constructor(page) {
    this.page = page;
    
    // ユーザー管理セクション
    this.userNameInput = '#user-name';
    this.userEmailInput = '#user-email';
    this.userAgeInput = '#user-age';
    this.addUserButton = 'button:has-text("ユーザー追加")';
    this.usersList = '#users-list';
    
    // TODO管理セクション
    this.todoTitleInput = '#todo-title';
    this.todoUserSelect = '#todo-user';
    this.addTodoButton = 'button:has-text("TODO追加")';
    this.todosList = '#todos-list';
  }

  async addUser(name, email, age) {
    await this.page.fill(this.userNameInput, name);
    await this.page.fill(this.userEmailInput, email);
    await this.page.fill(this.userAgeInput, age.toString());
    await this.page.click(this.addUserButton);
    await this.page.waitForTimeout(1000); // APIレスポンス待機
  }

  async addTodo(title, userName) {
    await this.page.fill(this.todoTitleInput, title);
    await this.page.selectOption(this.todoUserSelect, { label: userName });
    await this.page.click(this.addTodoButton);
    await this.page.waitForTimeout(1000); // APIレスポンス待機
  }

  async getUserByName(name) {
    const userElement = this.page.locator(`${this.usersList} .user-item:has-text("${name}")`);
    return userElement;
  }

  async getTodoByTitle(title) {
    const todoElement = this.page.locator(`${this.todosList} .todo-item:has-text("${title}")`);
    return todoElement;
  }

  async deleteUser(name) {
    // 確認ダイアログを自動で承認
    this.page.on('dialog', dialog => dialog.accept());
    
    const userElement = await this.getUserByName(name);
    await userElement.locator('button:has-text("削除")').click();
    await this.page.waitForTimeout(1000);
  }

  async deleteTodo(title) {
    // 確認ダイアログを自動で承認
    this.page.on('dialog', dialog => dialog.accept());
    
    const todoElement = await this.getTodoByTitle(title);
    await todoElement.locator('button:has-text("削除")').click();
    await this.page.waitForTimeout(1000);
  }

  async toggleTodo(title) {
    const todoElement = await this.getTodoByTitle(title);
    await todoElement.locator('input[type="checkbox"]').click();
    await this.page.waitForTimeout(1000);
  }

  async isUserVisible(name) {
    try {
      const userElement = await this.getUserByName(name);
      return await userElement.isVisible();
    } catch (error) {
      return false;
    }
  }

  async isTodoVisible(title) {
    try {
      const todoElement = await this.getTodoByTitle(title);
      return await todoElement.isVisible();
    } catch (error) {
      return false;
    }
  }

  async isTodoCompleted(title) {
    const todoElement = await this.getTodoByTitle(title);
    const isCompleted = await todoElement.locator('input[type="checkbox"]').isChecked();
    return isCompleted;
  }

  async getUsersCount() {
    const users = await this.page.locator(`${this.usersList} .user-item`).count();
    return users;
  }

  async getTodosCount() {
    const todos = await this.page.locator(`${this.todosList} .todo-item`).count();
    return todos;
  }
}

module.exports = HomePage;