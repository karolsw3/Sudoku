
import scrypt from 'scrypt-js'

export default ({
  methods: {
    derivePassword (password, errorCallback, successCallback) {
      var salt = Buffer.from('Sudoku')
      scrypt(Buffer.from(password), salt, Math.pow(2, 14), 8, 1, 64, (error, progress, key) => {
        if (error) {
          errorCallback('Error with password derivation')
        } else if (key) {
          password = ''
          key.map((item) => {
            if (item.toString(16).length === 1) {
              password += '0'
            }
            password += item.toString(16)
          })
          successCallback(password)
        }
      })
    }
  }
})
