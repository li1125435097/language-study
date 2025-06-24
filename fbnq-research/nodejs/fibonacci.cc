#include <node_api.h>

// 递归计算斐波那契数
int fibonacci(int n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
// Node-API包装函数
napi_value Fibonacci(napi_env env, napi_callback_info info) {
  size_t argc = 1;
  napi_value args[1];
  napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
  int n;
  napi_get_value_int32(env, args[0], &n);
  int result = fibonacci(n);
  napi_value return_value;
  napi_create_int32(env, result, &return_value);
  return return_value;
}

// 模块初始化
napi_value Init(napi_env env, napi_value exports) {
  napi_property_descriptor desc = {
    "fibonacci",
    nullptr,
    Fibonacci,
    nullptr,
    nullptr,
    nullptr,
    napi_default,
    nullptr
  };
  napi_define_properties(env, exports, 1, &desc);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
