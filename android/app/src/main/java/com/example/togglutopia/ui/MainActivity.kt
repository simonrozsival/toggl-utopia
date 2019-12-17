package com.example.togglutopia.ui

import android.os.Bundle
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.ui.core.setContent
import com.example.togglutopia.utils.startTimer


class MainActivity : AppCompatActivity() {

    private val viewModel by viewModels<MainViewModel>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        startTimer()
        setContent {
            TogglApp(viewModel)
        }
    }

    override fun onBackPressed() {
        when (TogglState.currentScreen) {
            Screen.MainLog -> super.onBackPressed()
            is Screen.EditTimeEntry -> navigateTo(Screen.MainLog)
        }
    }
}

