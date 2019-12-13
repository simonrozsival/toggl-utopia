package com.example.togglutopia.ui

import android.os.Bundle
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.Observer
import androidx.ui.core.setContent
import kotlinx.coroutines.ExperimentalCoroutinesApi

class MainActivity : AppCompatActivity() {

    private val viewModel by viewModels<MainViewModel>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            TogglApp(viewModel)
        }
    }

    override fun onBackPressed() {
        when (TogglState.currentScreen) {
            Screen.Log -> super.onBackPressed()
            is Screen.Edit -> navigateTo(Screen.Log)
        }
    }
}

