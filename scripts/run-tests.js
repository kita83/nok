const { spawn } = require('child_process');
const path = require('path');

let serverProcess;
let testProcess;

// サーバーを起動
function startServer() {
  return new Promise((resolve, reject) => {
    console.log('🚀 サーバーを起動中...');
    serverProcess = spawn('node', ['src/app.js'], {
      stdio: 'pipe',
      cwd: path.resolve(__dirname, '..')
    });

    serverProcess.stdout.on('data', (data) => {
      const output = data.toString();
      console.log(`[SERVER] ${output.trim()}`);
      if (output.includes('サーバーがポート')) {
        console.log('✅ サーバーが起動しました');
        setTimeout(resolve, 1000); // サーバーの完全起動を待つ
      }
    });

    serverProcess.stderr.on('data', (data) => {
      console.error(`[SERVER ERROR] ${data.toString().trim()}`);
    });

    serverProcess.on('error', (error) => {
      console.error('❌ サーバーの起動に失敗しました:', error);
      reject(error);
    });

    // タイムアウト設定
    setTimeout(() => {
      reject(new Error('サーバーの起動がタイムアウトしました'));
    }, 10000);
  });
}

// テストを実行
function runTests() {
  return new Promise((resolve, reject) => {
    console.log('🧪 テストを実行中...');
    testProcess = spawn('npm', ['test'], {
      stdio: 'inherit',
      cwd: path.resolve(__dirname, '..')
    });

    testProcess.on('close', (code) => {
      if (code === 0) {
        console.log('✅ すべてのテストが成功しました');
        resolve();
      } else {
        console.log(`❌ テストが失敗しました (終了コード: ${code})`);
        reject(new Error(`テストが失敗しました (終了コード: ${code})`));
      }
    });

    testProcess.on('error', (error) => {
      console.error('❌ テストの実行に失敗しました:', error);
      reject(error);
    });
  });
}

// クリーンアップ
function cleanup() {
  console.log('🧹 クリーンアップ中...');
  if (serverProcess) {
    serverProcess.kill();
    console.log('🛑 サーバーを停止しました');
  }
  if (testProcess) {
    testProcess.kill();
  }
}

// メイン処理
async function main() {
  try {
    await startServer();
    await runTests();
  } catch (error) {
    console.error('❌ エラーが発生しました:', error.message);
    process.exit(1);
  } finally {
    cleanup();
  }
}

// シグナルハンドリング
process.on('SIGINT', () => {
  console.log('\n🛑 テストが中断されました');
  cleanup();
  process.exit(0);
});

process.on('SIGTERM', () => {
  console.log('\n🛑 テストが終了されました');
  cleanup();
  process.exit(0);
});

// 実行
main();