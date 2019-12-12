package com.example.togglutopia.utils

import java.text.ParseException
import java.text.SimpleDateFormat
import java.util.*


class DateExtensions {
}

/**
 * Created by semanticer on 2. 5. 2016.
 */
fun Calendar.getISO8601(): String {
    return ISO8601.fromCalendar(this)
}

object ISO8601 {
    /** Transform Calendar to ISO 8601 string.  */
    fun fromCalendar(calendar: Calendar): String {
        val date: Date = calendar.time
        val formatted: String = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ssZ")
                .format(date)
        return formatted.substring(0, 22) + ":" + formatted.substring(22)
    }

    /** Get current date and time formatted as ISO 8601 string.  */
    fun now(): String {
        return fromCalendar(GregorianCalendar.getInstance())
    }

    /** Transform ISO 8601 string to Calendar.  */
    @Throws(ParseException::class)
    fun toCalendar(iso8601string: String?): Calendar {
        var iso8601string = iso8601string
        if (iso8601string == null || iso8601string == "") {
            iso8601string = "1977-10-21T17:43:56+02:00" // give it invalid but nonnull value
        }
        val calendar: Calendar = GregorianCalendar.getInstance()
        var s = iso8601string.replace("Z", "+00:00")
        s = try {
            s.substring(0, 22) + s.substring(23) // to get rid of the ":"
        } catch (e: IndexOutOfBoundsException) {
            throw ParseException("Invalid length", 0)
        }
        val date: Date = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss").parse(s)
        calendar.time = date
        return calendar
    }

    @Throws(ParseException::class)
    fun shiftISO(isoFrom: String?, timeShiftInMillis: Long): String {
        val calendar: Calendar = toCalendar(isoFrom)
        val newMillis: Long = calendar.timeInMillis + timeShiftInMillis
        calendar.timeInMillis = newMillis
        return fromCalendar(calendar)
    }
}