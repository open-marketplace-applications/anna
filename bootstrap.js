import './static/reset.scss';
import './static/theme.scss';

import("./pkg").then(module => {
  module.run_app();
});
