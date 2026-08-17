// Exercises RTTI ops: dynamic_cast and typeid, which need vtable-based
// runtime type identification for polymorphic classes.
#include <typeinfo>

struct Base {
  virtual ~Base() {}
};
struct Derived : Base {};

Derived *do_cast(Base *b) { return dynamic_cast<Derived *>(b); }

const std::type_info &do_typeid(Base *b) { return typeid(*b); }
