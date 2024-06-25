package com.jetbrains.kmpapp.data

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ArticleObject(
    val id: String,
    val title: String,
    val content: String,
    val publishedDate: Instant
)
