#ifndef HOOKLENSWINDOW_H
#define HOOKLENSWINDOW_H

#include <QWidget>

class QPushButton;
class QLineEdit;
class QLabel;
class QVBoxLayout;
class QHBoxLayout;

class HookLensWindow : public QWidget
{
    Q_OBJECT

public:
    explicit HookLensWindow(QWidget *parent = nullptr);

signals:
    void fenStringReady(const QString& fen);
    void bestMoveReady(const QString& fromSquare, const QString& toSquare);
    void resetRequested();

private slots:
    void onAddPhotoClicked();
    void onSendClicked();
    void onBestMoveClicked();
    void onResetClicked();

private:
    QPushButton* m_addPhotoButton;
    QPushButton* m_sendButton;
    QPushButton* m_bestMoveButton;
    QPushButton* m_resetButton;
    QLineEdit* m_pathLineEdit;
    QLineEdit* m_resultLineEdit;
    QLineEdit* m_fromSquareEdit;
    QLineEdit* m_toSquareEdit;
    QLabel* m_imageLabel;
    QVBoxLayout* m_mainLayout;
};

#endif // HOOKLENSWINDOW_H
