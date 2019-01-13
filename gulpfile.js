// var gulp = require('gulp'),
//     inlineCss = require('gulp-inline-css');
//
// gulp.task('default', function() {
//     return gulp.src('./static/src/*.html')
//         .pipe(inlineCss())
//         .pipe(gulp.dest('./static/build/'));
// });

var gulp = require('gulp');
var inline = require('gulp-inline')
  , uglify = require('gulp-uglify')
  , minifyCss = require('gulp-minify-css')
  , autoprefixer = require('gulp-autoprefixer');
var inlineimg = require('gulp-inline-image-html');
var inlinesource = require('gulp-inline-source');

gulp.task('default', function() {
    return gulp.src('./static/src/index.html')
      .pipe(inline({
        base: './static/',
        // js: uglify,
        // css: [minifyCss, autoprefixer({ browsers:['last 2 versions'] })],
        // disabledTypes: ['svg', 'img'], // Only inline css files
        // ignore: ['./css/do-not-inline-me.css']
      }))
      .pipe(gulp.dest('./static/dist/'));
});

gulp.task('default2', function () {
  return gulp.src('static/src/*.html')
    .pipe(inlineimg('static/src'))  // takes in the directory to use as the root when looking for images
    .pipe(gulp.dest('static/dist'));
});

gulp.task('default3', function () {
    return gulp.src('./static/src/*.html')
        .pipe(inlinesource())
        .pipe(gulp.dest('./static/dist/'));
});
