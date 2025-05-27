const { setWorldConstructor, Before, After } = require('@cucumber/cucumber');
const { chromium } = require('playwright');

class CustomWorld {
  constructor() {
    this.browser = null;
    this.context = null;
    this.page = null;
    this.baseUrl = process.env.BASE_URL || 'http://localhost:3000';
  }

  async init() {
    this.browser = await chromium.launch({
      headless: process.env.HEADLESS !== 'false',
      slowMo: process.env.SLOW_MO ? parseInt(process.env.SLOW_MO) : 0
    });
    this.context = await this.browser.newContext({
      viewport: { width: 1280, height: 720 },
      ignoreHTTPSErrors: true
    });
    this.page = await this.context.newPage();
  }

  async cleanup() {
    if (this.page) await this.page.close();
    if (this.context) await this.context.close();
    if (this.browser) await this.browser.close();
  }

  async goto(path = '') {
    const url = `${this.baseUrl}${path}`;
    await this.page.goto(url);
    await this.page.waitForLoadState('networkidle');
  }

  async waitForElement(selector, timeout = 5000) {
    return await this.page.waitForSelector(selector, { timeout });
  }

  async clickElement(selector) {
    await this.page.click(selector);
  }

  async fillInput(selector, value) {
    await this.page.fill(selector, value);
  }

  async selectOption(selector, value) {
    await this.page.selectOption(selector, value);
  }

  async getText(selector) {
    return await this.page.textContent(selector);
  }

  async isVisible(selector) {
    return await this.page.isVisible(selector);
  }

  async screenshot(name) {
    await this.page.screenshot({ 
      path: `reports/screenshots/${name}.png`,
      fullPage: true 
    });
  }
}

setWorldConstructor(CustomWorld);

Before(async function() {
  await this.init();
});

After(async function(scenario) {
  if (scenario.result.status === 'FAILED') {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    await this.screenshot(`failed-${scenario.pickle.name}-${timestamp}`);
  }
  await this.cleanup();
});