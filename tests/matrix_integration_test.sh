#!/bin/bash

echo "🧪 nok Matrix Integration Tests"
echo "==============================="

# テスト結果を記録する変数
TESTS_PASSED=0
TESTS_FAILED=0

# テスト関数
run_test() {
    local test_name="$1"
    local test_command="$2"

    echo -n "Testing: $test_name... "

    if eval "$test_command" >/dev/null 2>&1; then
        echo "✅ PASSED"
        ((TESTS_PASSED++))
    else
        echo "❌ FAILED"
        ((TESTS_FAILED++))
    fi
}

# 1. Conduitサーバー動作確認
echo -e "\n📡 Phase 1: Server Connectivity Tests"
run_test "Conduit API response" "curl -s http://localhost:6167/_matrix/client/versions | grep -q versions"

# 2. ユーザー認証テスト
echo -e "\n🔐 Phase 2: Authentication Tests"
# test1ユーザーのログインテスト（Matrix SDKクライアントとして）
TEMP_OUTPUT=$(mktemp)
run_test "test1 user login" "printf 'test1\ndemo1234\n5\n' | ./target/debug/test_matrix > $TEMP_OUTPUT 2>&1 && grep -q 'Login successful' $TEMP_OUTPUT"
rm -f $TEMP_OUTPUT

# 3. ルーム機能テスト
echo -e "\n🏠 Phase 3: Room Functionality Tests"
run_test "Room existence check" "curl -s -X GET 'http://localhost:6167/_matrix/client/r0/directory/room/%23general%3Anok.local' | grep -q 'room_id'"

# 4. メッセージ機能テスト
echo -e "\n💬 Phase 4: Messaging Tests"
# テスト用の自動化メッセージ送信
# 実際のMatrix SDKを使って、test2ユーザーでルーム参加→メッセージ送信のテスト

# test2ユーザーでルーム参加とメッセージ送信を一連でテスト
TEST_MSG="Hello from integration test - $(date)"
TEMP_TEST_OUTPUT=$(mktemp)

# 複雑なインタラクションテスト用の入力を作成
echo "test2
demo1234
1
#general:nok.local
2
1
$TEST_MSG
5" > /tmp/test_input

run_test "test2 room join and message" "cat /tmp/test_input | ./target/debug/test_matrix > $TEMP_TEST_OUTPUT 2>&1 && grep -q 'Message sent' $TEMP_TEST_OUTPUT"

rm -f $TEMP_TEST_OUTPUT /tmp/test_input

# 結果レポート
echo -e "\n📊 Test Results Summary"
echo "======================="
echo "✅ Tests Passed: $TESTS_PASSED"
echo "❌ Tests Failed: $TESTS_FAILED"
echo "📈 Success Rate: $(( TESTS_PASSED * 100 / (TESTS_PASSED + TESTS_FAILED) ))%"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n🎉 All tests passed! Matrix integration is working correctly."
    exit 0
else
    echo -e "\n⚠️  Some tests failed. Check the implementation."
    exit 1
fi