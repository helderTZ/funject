#include <cstdio>

int foo(int a, int b);
void bar();

class FooBar {
public:
    FooBar() = default;

private:
    void bazz();
};

struct BarFoo {
public:
    BarFoo() = default;

private:
    void bazz();
};

template <typename T>
T get_foo(T t) {
    return t;
}

namespace deadbeef {
    void dead();

    void beef() {
        printf("deadbeef::beef\n");
    }
}

// --------- DEFINITIONS --------- //

int foo(int a, int b) {
    printf("foo\n");
    return 0;
}

void bar() {
    printf("bar\n");
}

void FooBar::bazz() {
    printf("baz\n");
}

void BarFoo::bazz() {
    printf("baz\n");
}

void deadbeef::dead() {
    printf("deadbeef::dead\n");
}