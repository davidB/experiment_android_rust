package com.example.myapplication

import org.junit.Test

import org.junit.Assert.*

/**
 * See [testing documentation](http://d.android.com/tools/testing).
 * This test require to define java.library.path (Android Studio doesn't use the conf from gradle)
 */
class RustGreetingsTest {
    @Test
    fun sayHello_isCorrect() {
        System.err.println(System.getProperty("java.library.path"))
//        System.err.println(System.getenv("LD_LIBRARY_PATH"))
        val sut = RustGreetings()
        assertEquals("Hello Foo", sut.sayHello("Foo"))
    }
}
