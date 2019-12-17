package com.example.togglutopia.ui.login

import android.content.ContentValues.TAG
import android.util.Log
import androidx.compose.*
import androidx.ui.core.Modifier
import androidx.ui.core.Text
import androidx.ui.core.TextField
import androidx.ui.core.dp
import androidx.ui.layout.Column
import androidx.ui.layout.Padding
import androidx.ui.material.Button
import androidx.ui.material.MaterialTheme
import androidx.ui.material.surface.Card
import androidx.ui.tooling.preview.Preview
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
import com.example.togglutopia.ui.LoginInteractions

@Model
class LoginState(var username: String, var password: String)

@Composable
fun LoginScreen(interactions: LoginInteractions) {
    val loginState = +memo { LoginState("valenta@test.com", "testtest") }
    Container() {
        Card(elevation = 4.dp, modifier = Height(200.dp) wraps Expanded wraps Spacing(left = 8.dp, right = 8.dp)) {
            Padding(padding = 16.dp) {
                Column {
                    Text(text = "Login", style = ((+MaterialTheme.typography()).h6))
                    HeightSpacer(height = 16.dp)
                    AuthEditTextField(loginState.username) { loginState.username = it }
                    HeightSpacer(height = 1.dp)
                    AuthEditTextField(loginState.password) { loginState.password = it }
                    HeightSpacer(height = 16.dp)
                    Button(text = "Login", onClick = { interactions.login(loginState.username, loginState.password) })
                }
            }
        }
    }
}

@Composable
fun AuthEditTextField(initValue: String, onValueChange: (String) -> Unit) {
    Card(elevation = 1.dp) {
        Padding(padding = 6.dp) {
            TextField(
                    value = initValue,
                    onValueChange = onValueChange,
                    textStyle = ((+MaterialTheme.typography()).body2)
            )
        }
    }
}

@Preview
@Composable
fun preview() {
    LoginScreen(object : LoginInteractions {
        override fun login(username: String, password: String) {}
    })
}