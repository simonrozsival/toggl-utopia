package com.example.togglutopia.ui.edit

import androidx.compose.Composable
import androidx.ui.core.Text
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry

@Composable
fun EditScreen(timeEntry: TimeEntry) {
    Text(text = timeEntry.description)
}

