
void add(int *x)
{
    *x = *x + 10;
}

int main()
{

    int x = 100;
    add(&x);

    return 0;
}