package com.example.myapplication;

public class RustGreetings {
    static {
        System.loadLibrary("rust");
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}
