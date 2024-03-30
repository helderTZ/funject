int foo(int a, int b);
void bar();

class FooBar {
public:
    FooBar() = default;

private:
    void bazz();
};

template <typename T>
T get_foo(T t);
