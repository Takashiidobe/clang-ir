// Exercises cir.base_class_addr / cir.derived_class_addr: pointer adjustment
// for non-virtual upcasts and downcasts (static_cast) between related
// classes with a non-zero base-subobject offset.
struct Base {
  int b;
};
struct Other {
  int o;
};
struct Derived : Other, Base {
  int d;
};

Base *up_cast(Derived *d) { return d; }

Derived *down_cast(Base *b) { return static_cast<Derived *>(b); }
