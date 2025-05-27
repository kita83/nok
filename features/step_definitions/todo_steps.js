const { Given, When, Then } = require('@cucumber/cucumber');
const { expect } = require('chai');
const HomePage = require('../../src/pages/HomePage');

Given('TODO一覧に {string} が存在する', async function (todoTitle) {
  const isVisible = await this.homePage.isTodoVisible(todoTitle);
  expect(isVisible).to.be.true;
});

Given('TODO一覧に {string} が完了状態で存在する', async function (todoTitle) {
  const isVisible = await this.homePage.isTodoVisible(todoTitle);
  const isCompleted = await this.homePage.isTodoCompleted(todoTitle);
  expect(isVisible).to.be.true;
  expect(isCompleted).to.be.true;
});

When('ユーザーが {string} というTODOを {string} に追加する', 
  async function (todoTitle, userName) {
    await this.homePage.addTodo(todoTitle, userName);
  }
);

When('ユーザーが {string} を完了状態にする', async function (todoTitle) {
  const isCompleted = await this.homePage.isTodoCompleted(todoTitle);
  if (!isCompleted) {
    await this.homePage.toggleTodo(todoTitle);
  }
});

When('ユーザーが {string} を未完了状態にする', async function (todoTitle) {
  const isCompleted = await this.homePage.isTodoCompleted(todoTitle);
  if (isCompleted) {
    await this.homePage.toggleTodo(todoTitle);
  }
});

When('ユーザーが {string} というTODOを削除する', async function (todoTitle) {
  await this.homePage.deleteTodo(todoTitle);
});

Then('TODO一覧に {string} が表示される', async function (todoTitle) {
  const isVisible = await this.homePage.isTodoVisible(todoTitle);
  expect(isVisible).to.be.true;
});

Then('TODO一覧に {string} が表示されない', async function (todoTitle) {
  const isVisible = await this.homePage.isTodoVisible(todoTitle);
  expect(isVisible).to.be.false;
});

Then('{string} が完了状態として表示される', async function (todoTitle) {
  const isCompleted = await this.homePage.isTodoCompleted(todoTitle);
  expect(isCompleted).to.be.true;
});

Then('{string} が未完了状態として表示される', async function (todoTitle) {
  const isCompleted = await this.homePage.isTodoCompleted(todoTitle);
  expect(isCompleted).to.be.false;
});

Then('TODO数が {int} である', async function (expectedCount) {
  const actualCount = await this.homePage.getTodosCount();
  expect(actualCount).to.equal(expectedCount);
});