
void add(int *x)
{
    *x = *x + 10;
}

int main()
{

    int x = 100;
    int y = 1;

    add(&x);
    add(&y);

    return 0;
}