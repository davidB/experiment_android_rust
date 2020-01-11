package com.example.myapplication;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

public class MainActivity extends Activity implements SurfaceHolder.Callback {

    private static final String TAG = "MainActivity";
//    private SurfaceView surfaceView;
    private long androidActivityProxy;

    @Override
    public void onCreate(Bundle savedInstanceState) {
        Log.v(TAG, "MainActivity::onCreate()");
        super.onCreate(savedInstanceState);
        SurfaceView surfaceView = new SurfaceView(this);
        surfaceView.getHolder().addCallback(this);
        setContentView(surfaceView);
        androidActivityProxy = LibRust.onCreate(this);
    }

    @Override
    public void onStart() {
        Log.v(TAG, "MainActivity::onStart()");
        super.onStart();
        LibRust.onStart(androidActivityProxy);
    }

    @Override
    public void onResume() {
        Log.v(TAG, "MainActivity::onResume()");
        super.onResume();
        LibRust.onResume(androidActivityProxy);
    }

    @Override
    public void onPause() {
        Log.v(TAG, "MainActivity::onPause()");
        super.onPause();
        LibRust.onPause(androidActivityProxy);
    }

    @Override
    public void onStop() {
        Log.v(TAG, "MainActivity::onStop()");
        super.onStop();
        LibRust.onStop(androidActivityProxy);
    }

    @Override
    public void onDestroy() {
        Log.v(TAG, "MainActivity::onDestroy()");
        LibRust.onDestroy(androidActivityProxy);
        super.onDestroy();
        androidActivityProxy = 0;
    }

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        Log.v(TAG, "MainActivity::surfaceCreated()");
        if (androidActivityProxy != 0) {
            LibRust.surfaceCreated(androidActivityProxy, holder.getSurface());
        }
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        Log.v(TAG, "MainActivity::surfaceChanged()");
        if (androidActivityProxy != 0) {
            LibRust.surfaceChanged(androidActivityProxy, holder.getSurface());
        }
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        Log.v(TAG, "MainActivity::surfaceDestroyed()");
        if (androidActivityProxy != 0) {
            LibRust.surfaceDestroyed(androidActivityProxy);
        }
    }

}
