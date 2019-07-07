<template lang="pug">
    #app
        div(style="min-height:150px")
        .container.bg-white.p-5
            .row.justify-content-center
                .col-12.p-2
                    img.d-block.img-fluid.mx-auto.w-25(src="./assets/images/libra-utils.png" alt="Libra")
            .row
                .col-12
                    hr
            .row.justify-content-center
                .col-7.text-center
                    form.p-5.bg-info.shadow
                        h3.pb-2 Generate Account
                        hr.pb-2
                        .form-group
                            label(for="publicAddress").font-raleway Public Address
                            input.form-control(type="text" id="publicAddress" placeholder="Libra Public Account Address" readonly)
                        .form-group
                            label(for="mnemonic").font-raleway Mnemonic
                            .input-group
                                input.form-control(type="text" id="mnemonic" placeholder="Account Mnemonic" readonly)
                                .input-group-append
                                    .btn.btn-secondary Reveal
        Footer
</template>

<script>
import Footer from "./Components/Footer";

export default {
  name: "app",
  components: {
    Footer
  }
};
</script>