package com.example.togglutopia.ui.log

import androidx.compose.Composable
import androidx.compose.ambient
import androidx.compose.unaryPlus
import androidx.ui.core.ContextAmbient
import androidx.ui.core.Text
import androidx.ui.core.dp
import androidx.ui.foundation.Clickable
import androidx.ui.foundation.VerticalScroller
import androidx.ui.layout.*
import androidx.ui.material.Button
import androidx.ui.material.ripple.Ripple
import androidx.ui.material.surface.Card
import androidx.ui.material.surface.Surface
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.LogInteractions
import com.example.togglutopia.ui.Screen
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.navigateTo
import com.example.togglutopia.ui.timeEntry.TimeEntryContent
import java.util.*

@Composable
fun LogScreen(interactions: LogInteractions, currentTime: Long) {
    Column {
        VerticalScroller(modifier = Flexible(1f)) {
            Column(modifier = Spacing(left = 8.dp, right = 8.dp)) {
                HeightSpacer(height = 16.dp)
                StatusInfo(interactions)

                TogglState.timeEntryList.filter { it.duration != null }.sortedByDescending { it.start }.forEach { te ->
                    TimeEntryRow(timeEntry = te)
                }
            }
        }
        val runningEntry = TogglState.timeEntryList.find { it.duration == null }
        if (runningEntry != null) {
            BottomBar {
                TimeEntryContent(runningEntry)
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
private fun BottomBar(content: @Composable() () -> Unit) {
    Surface(elevation = 8.dp) {
        Container(modifier = Height(80.dp) wraps Expanded) {
            Row {
                content()
            }
        }
    }
}


@Composable
fun StatusInfo(interactions: LogInteractions) {
    Card(elevation = 16.dp, modifier = ExpandedWidth) {
        Padding(padding = 16.dp) {
            Column {
                Row{
                    Button(text = "Sync", onClick = { interactions.sync() })
                    val fullname = TogglState.user?.fullname ?: ""
                    Text(text = "You are logged in as $fullname", modifier = Spacing(8.dp))
                }

                HeightSpacer(height = 16.dp)

                Row {
                    Button(text = "Add TE manually", onClick = { TogglState.newTimeEntry() })
                    WidthSpacer(width = 16.dp)
                    Button(text = "Add Running TE", onClick = { TogglState.newRunningTimeEntry() })
                }
            }
        }
    }
}


@Preview
@Composable
fun preview() {
    LogScreen(object : LogInteractions {
        override fun sync() {
        }
    }, TogglState.currentTime)
}