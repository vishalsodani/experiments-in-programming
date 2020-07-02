int g = 100; 
int h;
static int s;

main(int argc, char *argv[])
{
    printf("%d", argc); //no of command line parameters
    printf("%s\n", argv[0]); //no of command line parameters
    int a = 1; int b;
    static int c = 3;
    b = 2;
    c = mysum(a,b);
    printf("sum=%d\n", c);
    printf("sum=%d\n", h);
} 

