package com.example.togglutopia.ui

import androidx.compose.Composable
import androidx.compose.unaryPlus
import androidx.ui.animation.Crossfade
import androidx.ui.material.MaterialTheme
import androidx.ui.material.surface.Surface
import com.example.togglutopia.ui.edit.EditScreen
import com.example.togglutopia.ui.log.LogScreen
import com.example.togglutopia.ui.login.LoginScreen

@Composable
fun TogglApp(interactions: MainInteractions) {
    MaterialTheme(
        colors = lightThemeColors,
        typography = themeTypography
    ) {
        if (TogglState.user == null) {
            LoginScreen(interactions)
        } else {
            AppContent(interactions)
        }
    }
}

@Composable
private fun AppContent(interactions: MainInteractions) {
    Crossfade(TogglState.currentScreen) { screen ->
        Surface(color = (+MaterialTheme.colors()).background) {
            when (screen) {
                is Screen.Log -> LogScreen(interactions)
                is Screen.Edit -> EditScreen(screen.timeEntry)
            }
        }
    }
}