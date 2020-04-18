var gulp = require('gulp')
var yaml = require('gulp-yaml')

gulp.task('yaml', async function(cb) {
  gulp
    .src('./**/*.yaml')
    .pipe(yaml({ safe: false, space: 2 }))
    .pipe(
      gulp.dest(function (f) {
        return f.base
      })
    )
})

gulp.task(
  'default',
  gulp.series('yaml', function(cb) {
    gulp.watch('./**/*.yaml', gulp.series('yaml'))
    cb()
  })
)
