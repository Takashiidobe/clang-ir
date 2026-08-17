// Exercises cir.try / cir.begin_catch / cir.end_catch / cir.init_catch_param
// / cir.resume: a try block with a typed catch and a catch-all.
int risky();

int f() {
  try {
    return risky();
  } catch (int e) {
    return e;
  } catch (...) {
    return -1;
  }
}
