// Exercises cir.vtable.get_vptr / cir.vtable.get_virtual_fn_addr, emitted
// for a call through a base-class pointer to an overridden virtual method.
struct Base {
  virtual int f() { return 1; }
  virtual ~Base() {}
};

struct Derived : Base {
  int f() override { return 2; }
};

int call_it(Base *b) { return b->f(); }
