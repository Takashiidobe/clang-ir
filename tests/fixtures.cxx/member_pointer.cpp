// Exercises pointer-to-member ops: calling through a pointer-to-member
// function (cir.get_method) and accessing through a pointer-to-data-member
// (cir.get_runtime_member).
struct Foo {
  int x;
  int f(int a) { return a + x; }
};

int call_method(Foo *object, int (Foo::*method)(int), int arg) {
  return (object->*method)(arg);
}

int access_data(Foo *object, int Foo::*member) {
  return object->*member;
}
