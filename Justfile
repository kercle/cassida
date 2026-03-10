[working-directory: 'web/frontend']
serve-frontend-dev:
    npm install
    npm run dev

eval-server:
    cargo run --bin eval-server

[parallel]
serve-dev: eval-server serve-frontend-dev

profile-integration-tests:
    cargo flamegraph -p symbolics --profile profiling --unit-test -- tests::integration_tests::test_full_processing_chain
