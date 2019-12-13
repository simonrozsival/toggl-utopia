package com.example.togglutopia.ui.log

import androidx.compose.Composable
import androidx.ui.core.Text
import androidx.ui.core.dp
import androidx.ui.foundation.Clickable
import androidx.ui.foundation.DrawImage
import androidx.ui.foundation.VerticalScroller
import androidx.ui.graphics.Image
import androidx.ui.layout.*
import androidx.ui.material.Button
import androidx.ui.material.ripple.Ripple
import androidx.ui.material.surface.Card
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.LogInteractions
import com.example.togglutopia.ui.Screen
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.navigateTo
import com.example.togglutopia.ui.timeEntry.TimeEntryContent

@Composable
fun LogScreen(interactions: LogInteractions) {
    VerticalScroller {
        Column(modifier = Spacing(16.dp)) {
            StatusInfo(interactions)

            TogglState.timeEntryList.sortedByDescending { it.start }.forEach { te ->
                TimeEntryRow(timeEntry = te)
            }

            if (TogglState.user?.default_workspace_id != null) {
                HeightSpacer(32.dp)
                Button(text = "Add TE", onClick = { TogglState.newTimeEntry() })
            }
        }
    }
}

@Composable
fun TimeEntryRow(timeEntry: TimeEntry) {
    HeightSpacer(12.dp)
    Ripple(bounded = true) {
        Clickable(onClick = {
            navigateTo(Screen.Edit(timeEntry.id))
        }) {
            Card(elevation = 4.dp) {
                TimeEntryContent(timeEntry)
            }
        }
    }

}


@Composable
fun StatusInfo(interactions: LogInteractions) {
    Row{
        Button(text = "Sync", onClick = { interactions.sync() })
        val fullname = TogglState.user?.fullname ?: ""
        Text(text = "You are logged in as $fullname", modifier = Spacing(8.dp))
    }
}


@Preview
@Composable
fun preview() {
    LogScreen(object : LogInteractions {
        override fun sync() {
            TODO("not implemented") //To change body of created functions use File | Settings | File Templates.
        }
    })
}