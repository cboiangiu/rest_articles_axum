package com.jetbrains.kmpapp

import com.jetbrains.kmpapp.data.ArticleRepository
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject

class KoinDependencies : KoinComponent {
    val articleRepository: ArticleRepository by inject()
}
