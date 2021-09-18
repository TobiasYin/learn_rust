//
// Created by TobiasYin on 2021/9/16.
//
#include "stdio.h"

typedef struct {
    int a;
    int b;
    int c;
    int d;
} Demo;

void hello_demo(Demo d) {
    printf("hello demo for c: %d, %d, %d, %d\n", d.a, d.b, d.c, d.d);
}