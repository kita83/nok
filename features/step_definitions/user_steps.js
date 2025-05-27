const { Given, When, Then } = require('@cucumber/cucumber');
const { expect } = require('chai');
const HomePage = require('../../src/pages/HomePage');

Given('ユーザーがホームページにアクセスしている', async function () {
  await this.goto('/');
  this.homePage = new HomePage(this.page);
});

Given('ユーザー一覧に {string} が存在する', async function (userName) {
  const isVisible = await this.homePage.isUserVisible(userName);
  expect(isVisible).to.be.true;
});

When('ユーザーが名前 {string}、メールアドレス {string}、年齢 {string} でユーザーを追加する', 
  async function (name, email, age) {
    await this.homePage.addUser(name, email, parseInt(age));
  }
);

When('ユーザーが {string} というユーザーを削除する', async function (userName) {
  await this.homePage.deleteUser(userName);
});

Then('ユーザー一覧に {string} が表示される', async function (userName) {
  const isVisible = await this.homePage.isUserVisible(userName);
  expect(isVisible).to.be.true;
});

Then('ユーザー一覧に {string} が表示されない', async function (userName) {
  const isVisible = await this.homePage.isUserVisible(userName);
  expect(isVisible).to.be.false;
});

Then('ユーザー数が {int} である', async function (expectedCount) {
  const actualCount = await this.homePage.getUsersCount();
  expect(actualCount).to.equal(expectedCount);
});