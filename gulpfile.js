var gulp = require('gulp')
var yaml = require('gulp-yaml')

gulp.task('yaml', function(cb) {
  gulp
    .src('./syntaxes/*.yaml')
    .pipe(yaml({ safe: false, space: 2 }))
    .pipe(
      gulp.dest(function (f) {
        return f.base
      })
    )
  cb()
})

gulp.task(
  'watch-yaml',
  gulp.series('yaml', function(cb) {
    gulp.watch('./syntaxes/*.yaml', gulp.series('yaml'))
    cb()
  })
)
