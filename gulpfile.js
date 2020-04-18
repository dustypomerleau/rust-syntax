var gulp = require('gulp')
var yaml = require('gulp-yaml')

task('yaml', function(cb) {
  src('*.yaml')
    .pipe(yaml({ safe: false, space: 2 }))
    .pipe(
      dest(function (f) {
        return f.base
      })
    )
})

task(
  'default',
  series('yaml', function(cb) {
    watch('*.yaml', series('yaml'))
    cb()
  })
)
