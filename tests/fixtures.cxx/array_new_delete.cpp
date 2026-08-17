// Exercises array new/delete with a non-trivial element type, which needs
// per-element constructor/destructor loops (cir.array.ctor/cir.array.dtor)
// and exception cleanup on a throwing constructor (cir.alloc.exception).
struct Widget {
  Widget();
  ~Widget();
  int x;
};

Widget *make_widgets(int n) { return new Widget[n]; }

void free_widgets(Widget *w) { delete[] w; }
