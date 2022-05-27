# Developer Guide

## Leading principles

- Consistency
  > Eg: follow existing patterns until it is necessary to change the pattern
- Reduce cognitive load
  > Eg: reduce the element of surprise
- Reduce toil
  > Eg: specs and manifests should be generated from sound code, not hand-written.

## Live development setup

> Todo: inner dev-loop

If not using tilt, you can do the following:

```sh
kubectl apply -k manifests/crds # load the CRDs
kubectl apply -k manifests/install # install the operator. actually for dev we want a different overlay that configures the controller for localstack (for aws anyway)
```

> todo: localstack. See <https://docs.localstack.cloud/aws/feature-coverage/> for SSM Parameters and SecretsManager Secrets
> todo: deploy some opentelemetry collector

```sh
kubectl apply -k manifests/examples # deploy examples
```


## Style guides

- Name things appropriately and unambiguously
- Follow the general [Rust formatting guidelines][rust-formatting-guidelines]

## Documentation

[The four kinds of documentation][four-kinds-of-documentation]

- Learning oriented (tutorials)
- Goal-oriented (how-to guides)
- Understanding-oriented (discussions)
- Information-oriented reference material (developer docs)

> Todo: add guidelines about what documentations need to be written and where.

## Review

### Reviewers should...

- Follow the [Google Engineering: Standard of Code Review][google-standard-of-code-review] to a high degree.
- Try not to leave Pull Requests unreviewed for too long
  > **Why?**
  > - It is off-putting to people who have taken precious time out of their day to contribute.
  > - We would't want to miss out on the contributor's future improvements.
  > - It holds back improvements from end-users.

### Developers should...

- Make the job easy for the reviewer.
  > Eg: write the Pull Request summary and details with the reviewer in mind. They might not be as intimate with the area of change as you are.
- Use [conventional commits] (commit message style)
  > **Why?**
  > - It encourages each commit to map to a specific intent.
  > - It is easy to quickly scan the git log for particular types of changes.
  > - It can also be useful for automating changelogs.
  > - For the sake of consistency.
- Try not to leave Pull Request comments/requests go unresolved for too long.
  > Because other's miss out on your awesome improvements.
  > It is understandable to not be available to resolve straight away, but some communication within some days with an idea of tha plan or request for other's to help could be a good guideline.


[rust-formatting-guidelines]: https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/advice.md#names
[four-kinds-of-documentation]: https://www.writethedocs.org/videos/eu/2017/the-four-kinds-of-documentation-and-why-you-need-to-understand-what-they-are-daniele-procida/
[google-standard-of-code-review]: https://google.github.io/eng-practices/review/reviewer/standard.html
