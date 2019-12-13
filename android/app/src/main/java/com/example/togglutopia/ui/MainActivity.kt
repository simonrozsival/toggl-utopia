package com.example.togglutopia.ui

import android.os.Bundle
import android.os.Handler
import android.util.Log
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity

import androidx.ui.core.setContent
import java.util.*


class MainActivity : AppCompatActivity() {

    private val viewModel by viewModels<MainViewModel>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            TogglApp(viewModel)
        }

        val handler = Handler()
        val timer = Timer(false)
        val timerTask: TimerTask = object : TimerTask() {
            override fun run() {
                handler.post {
                    TogglState.currentTime = Date().time
                    Log.d("TimerTask", TogglState.currentTime.toString())
                }
            }
        }
        timer.schedule(timerTask, 1000, 1000) // 1000 = 1 second.
    }

    override fun onBackPressed() {
        when (TogglState.currentScreen) {
            Screen.Log -> super.onBackPressed()
            is Screen.Edit -> navigateTo(Screen.Log)
        }
    }
}

